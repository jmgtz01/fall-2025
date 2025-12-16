#![allow(dead_code, unused_imports)]

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct FileStats {
    word_count: usize,
    line_count: usize,
    char_frequencies: HashMap<char, usize>,
    size_bytes: u64,
}

impl Default for FileStats {
    fn default() -> Self {
        Self {
            word_count: 0,
            line_count: 0,
            char_frequencies: HashMap::new(),
            size_bytes: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum ProcessingError {
    IoError(String),
    Utf8Error(String),
    Cancelled,
}

#[derive(Debug)] // removed Clone as ProcessingError might not need it, kept it simple
struct FileAnalysis {
    filename: String,
    stats: FileStats,
    errors: Vec<ProcessingError>,
    processing_time: Duration,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(Message::NewJob(job)) => {
                    job();
                }
                Ok(Message::Terminate) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Proper shutdown mechanism
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            let _ = self.sender.send(Message::Terminate);
        }

        // Join all threads
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

fn analyze_file(path: PathBuf) -> FileAnalysis {
    let start_time = Instant::now();
    let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let run_analysis = || -> Result<FileStats, ProcessingError> {
        let mut stats = FileStats::default();

        // 1. Open File
        let mut file = File::open(&path)
            .map_err(|e| ProcessingError::IoError(e.to_string()))?;

        // 2. Get Metadata
        let meta = file.metadata()
            .map_err(|e| ProcessingError::IoError(e.to_string()))?;
        stats.size_bytes = meta.len();

        // 3. Read Content
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| ProcessingError::Utf8Error(e.to_string()))?;

        // 4. Compute Stats
        stats.line_count = content.lines().count();
        stats.word_count = content.split_whitespace().count();
        for c in content.chars() {
            *stats.char_frequencies.entry(c).or_insert(0) += 1;
        }

        Ok(stats)
    };

    // Capture the result
    let (stats, errors) = match run_analysis() {
        Ok(s) => (s, Vec::new()),
        Err(e) => (FileStats::default(), vec![e]),
    };

    FileAnalysis {
        filename,
        stats,
        errors,
        processing_time: start_time.elapsed(),
    }
}

fn main() {
    // 1. Configuration
    // Requirement Update: "Support processing files from multiple directories"
    let directories = vec![
        "./books",
        // Add more directories here if needed, e.g., "./data",
    ];
    let num_threads = 4;

    println!("Starting Parallel File Processor with {} threads...", num_threads);

    // 2. Cancellation Handler
    // Since external crates (like ctrlc) are forbidden, we use a standard thread 
    // that listens for the 'Enter' key on stdin to trigger cancellation.
    let is_cancelled = Arc::new(AtomicBool::new(false));
    let cancel_flag = Arc::clone(&is_cancelled);
    
    thread::spawn(move || {
        println!("(Press [ENTER] at any time to cancel processing)");
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            println!("\nCancelling operation...");
            cancel_flag.store(true, Ordering::Relaxed);
        }
    });

    // 3. Scan Directories
    let mut paths = Vec::new();
    for dir in &directories {
        println!("Scanning directory: {}", dir);
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    paths.push(entry.path());
                }
            }
        } else {
            eprintln!("Warning: Could not read directory '{}'", dir);
        }
    }

    let total_files = paths.len();
    if total_files == 0 {
        println!("No files found in specified directories.");
        return;
    }

    println!("Found {} files. Dispatching to workers...", total_files);

    // 4. Initialize ThreadPool & Channel
    let pool = ThreadPool::new(num_threads);
    let (tx, rx) = mpsc::channel();

    // 5. Submit Jobs
    for path in paths {
        let tx = tx.clone();
        let is_cancelled = Arc::clone(&is_cancelled);

        pool.execute(move || {
            // Check cancellation before expensive work
            if is_cancelled.load(Ordering::Relaxed) {
                // Send a cancelled result back so we track it properly
                let _ = tx.send(FileAnalysis {
                    filename: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                    stats: FileStats::default(),
                    errors: vec![ProcessingError::Cancelled],
                    processing_time: Duration::new(0, 0),
                });
                return;
            }

            let analysis = analyze_file(path);
            let _ = tx.send(analysis);
        });
    }

    // Drop original sender to close channel when all workers are done
    drop(tx);

    // 6. Process Results
    let mut processed_count = 0;
    let mut success_count = 0;
    let mut total_words = 0;
    let mut total_duration = Duration::new(0, 0);

    for analysis in rx {
        // If cancelled via stdin, stop collecting results immediately
        if is_cancelled.load(Ordering::Relaxed) {
            println!("\nProcess aborted by user.");
            break;
        }

        processed_count += 1;

        if !analysis.errors.is_empty() {
            // Only print error if it wasn't a cancellation event
            if !matches!(analysis.errors[0], ProcessingError::Cancelled) {
                println!("\n[ERROR] {}: {:?}", analysis.filename, analysis.errors);
            }
        } else {
            success_count += 1;
            total_words += analysis.stats.word_count;
            total_duration += analysis.processing_time;
        }

        // Simple progress bar
        print!("\rProgress: {}/{} files processed", processed_count, total_files);
        io::stdout().flush().unwrap();
    }

    // 7. Final Report
    println!("\n\n=== Processing Complete ===");
    println!("Total Files Scanned: {}", total_files);
    println!("Files Processed:     {}", processed_count);
    println!("Successful:          {}", success_count);
    println!("Total Words:         {}", total_words);
    println!("Total Time (Thread): {:?}", total_duration);
}