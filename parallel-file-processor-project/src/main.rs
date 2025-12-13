#![allow(dead_code, unused_imports)]

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct FileStats {
    word_count: usize,
    line_count: usize,
    char_frequencies: HashMap<char, usize>,
    size_bytes: u64,
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
            self.sender.send(Message::Terminate).unwrap();
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
    let mut errors = Vec::new();

    // Default empty stats
    let mut stats = FileStats {
        word_count: 0,
        line_count: 0,
        char_frequencies: HashMap::new(),
        size_bytes: 0,
    };

    // Attempt to open and read file
    match File::open(&path) {
        Ok(mut file) => {
            match file.metadata() {
                Ok(meta) => stats.size_bytes = meta.len(),
                Err(e) => errors.push(ProcessingError::IoError(e.to_string())),
            }

            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => {
                    // Logic for analyzers
                    stats.line_count = content.lines().count();
                    stats.word_count = content.split_whitespace().count();
                    
                    for c in content.chars() {
                        *stats.char_frequencies.entry(c).or_insert(0) += 1;
                    }
                }
                Err(e) => errors.push(ProcessingError::Utf8Error(e.to_string())), // Usually means binary file
            }
        }
        Err(e) => errors.push(ProcessingError::IoError(e.to_string())),
    }

    FileAnalysis {
        filename,
        stats,
        errors,
        processing_time: start_time.elapsed(),
    }
}

fn main() {
    // 1. Setup Configuration
    let books_directory = "./books"; // Make sure this folder exists
    let num_threads = 4; // Configurable worker count
    
    println!("Starting Parallel File Processor with {} threads...", num_threads);
    println!("Scanning directory: {}", books_directory);

    // 2. Initialize ThreadPool
    let pool = ThreadPool::new(num_threads);

    // 3. Find files
    let paths = match fs::read_dir(books_directory) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file()) // simple filter, can be expanded to check extensions
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Error reading directory '{}': {}", books_directory, e);
            return;
        }
    };

    let total_files = paths.len();
    if total_files == 0 {
        println!("No files found. Please ensure you have downloaded books into '{}'.", books_directory);
        return;
    }

    println!("Found {} files. Processing...", total_files);

    // 4. Create a channel for results (Real-time updates)
    let (tx, rx) = mpsc::channel();
    
    // Shared state for cancellation (Constraint check: Support cancellation)
    let is_cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));

    // 5. Submit jobs
    for path in paths {
        let tx = tx.clone();
        let is_cancelled = Arc::clone(&is_cancelled);
        
        pool.execute(move || {
            // Check cancellation before starting
            if is_cancelled.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }
            
            let analysis = analyze_file(path);
            
            // Send result back to main thread
            let _ = tx.send(analysis);
        });
    }

    // Drop the original sender so the receiver loop knows when all workers are done
    drop(tx);

    // 6. Process Results (Real-time progress updates)
    let mut processed_count = 0;
    let mut total_words = 0;
    let mut total_duration = Duration::new(0, 0);

    // This loop blocks until all senders (workers) are dropped
    for analysis in rx {
        processed_count += 1;

        // Progress UI
        if !analysis.errors.is_empty() {
            println!("[ERROR] {}: {:?}", analysis.filename, analysis.errors);
        } else {
            // Uncomment the line below for verbose per-file status
            // println!("[OK] {} ({:?}) - {} words", analysis.filename, analysis.processing_time, analysis.stats.word_count);
        }

        total_words += analysis.stats.word_count;
        total_duration += analysis.processing_time;

        // Simple progress bar
        print!("\rProgress: {}/{} files processed", processed_count, total_files);
        use std::io::Write;
        io::stdout().flush().unwrap();
    }

    // 7. Final Statistics
    println!("\n\n=== Processing Complete ===");
    println!("Total Files: {}", processed_count);
    println!("Total Words Counted: {}", total_words);
    println!("Total Processing Time (Sum of threads): {:?}", total_duration);
}