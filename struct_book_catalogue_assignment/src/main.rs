use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(&filename).expect("Could not create file.");

    for book in books.iter()
    {
        let line = format!("{} {} {}", book.title, book.author, book.year);

        writeln!(file, "{}", line).expect("Could not write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).expect("Could not open file.");
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    
    for line_result in reader.lines() 
    {
        let line = line_result.unwrap(); 
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() 
        {
            continue;
        }

        
        let parts: Vec<&str> = trimmed_line.split_whitespace().collect();

        
        if parts.len() >= 3 {
            
            let year_str = parts[parts.len() - 1];
            
            
            let title = parts[0].to_string(); 
            let author = parts[1].to_string(); 
                        
            let year: u16 = year_str.parse().unwrap(); 

            books.push(Book {
                title,
                author,
                year,
            }); 
        }
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}