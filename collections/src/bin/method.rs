use core::fmt;

struct Book {
    title: String,
    pages: u32,
}

impl Book {
    // Method to calculate reading time assuming 1 page per minute
    fn reading_time(&self) -> u32 {
        self.pages
    }

    // Method to check if one book has more pages then the other
    fn has_more_pages_than(&self, other: &Book) -> bool {
        self.pages > other.pages
    }

    // Associated function (not a method) to create a book with default title and given pages
    fn default_book(pages: u32) -> Book {
        Book {
            title: String::from("Untitled"),
            pages,
        }
    }

    fn set_title(&mut self, new_title: String) {
        self.title = new_title
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Title: '{}' with {} pages", self.title, self.pages)
    }
}

fn main() {
    let book1 = Book {
        title: String::from("The Rust programming language"),
        pages: 500,
    };

    let book2 = Book {
        title: String::from("Learning Rust"),
        pages: 300,
    };

    let book3 = Book {
        title: String::from("Rust in action"),
        pages: 600,
    };

    let mut book4 = Book {
        title: String::from("New book"),
        pages: 100,
    };

    println!(
        "The reading time for '{}' is {} minutes.",
        book1.title,
        book1.reading_time()
    );
    println!(
        "Does '{}' have more pages than '{}'? {}",
        book1.title,
        book2.title,
        book1.has_more_pages_than(&book2)
    );
    println!(
        "Does '{}' have more pages than '{}'? {}",
        book1.title,
        book3.title,
        book1.has_more_pages_than(&book3)
    );

    let default_book = Book::default_book(150);
    println!(
        "Default book: '{}' with {} pages",
        default_book.title, default_book.pages
    );

    println!("Book4: {}", book4);

    book4.set_title(String::from("Updated title"));

    println!("Book4: {}", book4);
}
