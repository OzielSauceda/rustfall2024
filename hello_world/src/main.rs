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
    let mut file = File::create(filename).unwrap();

    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year);
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let mut totalBooks = Vec::new();
    let file = File::open(filename).unwrap();
    let readBook = BufReader::new(file);

    for bookLine in readBook.lines() {
        let book_info = bookLine.unwrap();
        let partOfBook: Vec<&str> = book_info.split(',').collect();

        totalBooks.push(Book{ 
            title: partOfBook[0].to_string(),
            author: partOfBook[1].to_string(),
            year: partOfBook[2].parse().unwrap(),
        });

    }
    return totalBooks;
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