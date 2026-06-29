mod book;

use crate::library::book::Searchable;
pub use book::{Book, Genre, Status};
use std::fmt;

/// Stores the book collection and manages library operations.
pub(crate) struct Library {
    books: Vec<Book>,
    next_id: u16,
}

/// Represents the possible errors when borrowing or returning a book.
#[derive(Debug)]
pub enum LibraryError {
    BookNotFound,
    AlreadyAvailable,
    AlreadyBorrowed,
}

impl Library {
    /// Creates a new library and assigns IDs to the initial books.
    /// # Arguments
    /// * `books` - Initial books to add to the library.
    /// # Returns
    /// A `Library` containing the provided books with assigned IDs.
    pub fn new(books: Vec<Book>) -> Self {
        let mut library = Self {
            books: Vec::new(),
            next_id: 0,
        };

        for book in books {
            library.add_book(book);
        }

        library
    }

    /// Returns all books currently stored in the library.
    /// # Returns
    /// A slice containing all books in the library.
    pub fn books(&self) -> &[Book] {
        &self.books
    }

    /// Adds a book to the library, assigns it the next available ID,
    /// and returns a reference to the added book.
    /// # Arguments
    /// * `book` - The book to add to the library.
    /// # Returns
    /// A reference to the book that was added.
    pub(crate) fn add_book(&mut self, mut book: Book) -> &Book {
        let id = self.next_id;

        book.set_id(id);
        self.books.push(book);

        self.next_id += 1;

        // Safe because a book was just pushed into the vector.
        self.books.last().unwrap()
    }

    /// Search for books by title. The search is case-insensitive and will return all books that
    /// contain the search term in their title.
    /// # Arguments
    /// * `title` - The title or part of the title to search for.
    /// # Returns
    /// A vector of references to the books that match the search criteria.
    pub(crate) fn search_by_title(&self, title: &str) -> Vec<&Book> {
        let title = title.trim().to_lowercase();

        self.books
            .iter()
            .filter(|book| book.matches_title(&title))
            .collect()
    }

    /// Searches for a mutable book reference by its ID.
    /// # Arguments
    /// * `id` - The ID of the book to find.
    /// # Returns
    /// A mutable reference to the matching book, or a `LibraryError`.
    fn search_by_id_mut(&mut self, id: u16) -> Result<&mut Book, LibraryError> {
        self.books
            .iter_mut()
            .find(|book| book.id() == id)
            .ok_or(LibraryError::BookNotFound)
    }

    /// Borrows a book by its ID.
    /// If the book is available, it is marked as borrowed and returned.
    /// If the book is already borrowed or cannot be found, an error is returned.
    /// # Arguments
    /// * `id` - The ID of the book to borrow.
    /// # Returns
    /// A result containing a reference to the borrowed book, or a `LibraryError`.
    pub(crate) fn borrow_book(&mut self, id: u16) -> Result<&Book, LibraryError> {
        let book_to_borrow: &mut Book = self.search_by_id_mut(id)?;

        match book_to_borrow.status() {
            Status::Available => {
                book_to_borrow.set_status(Status::Borrowed);
                Ok(book_to_borrow)
            }
            Status::Borrowed => Err(LibraryError::AlreadyBorrowed),
        }
    }

    /// Returns a borrowed book by its ID.
    /// If the book is borrowed, it is marked as available and returned.
    /// If the book is already available or cannot be found, an error is returned.
    /// # Arguments
    /// * `id` - The ID of the book to return.
    /// # Returns
    /// A result containing a reference to the returned book, or a `LibraryError`.
    pub(crate) fn return_book(&mut self, id: u16) -> Result<&Book, LibraryError> {
        let book_to_return = self.search_by_id_mut(id)?;

        match book_to_return.status() {
            Status::Borrowed => {
                book_to_return.set_status(Status::Available);
                Ok(book_to_return)
            }
            Status::Available => Err(LibraryError::AlreadyAvailable),
        }
    }

    /// Gets statistics about the library, including the total number of books, total pages,
    /// mean pages per book, total available books and total borrowed books.
    /// # Returns
    /// A tuple of u16 containing the total number of books, total pages, mean pages per book,
    /// total available books and total borrowed books.
    pub(crate) fn get_stats(&self) -> (u16, u16, u16, u16, u16) {
        // Normally, I would have returned a struct, but for the TP I had to return a tuple.
        let total_books = self.books().len() as u16;
        let total_pages: u16 = self.books().iter().map(|book| book.page_count()).sum();

        let mean_pages = total_pages.checked_div(total_books).unwrap_or(0);

        let total_available_books = self
            .books()
            .iter()
            .filter(|book| matches!(book.status(), &Status::Available))
            .count() as u16;
        let total_borrowed_books = self
            .books()
            .iter()
            .filter(|book| matches!(book.status(), &Status::Borrowed))
            .count() as u16;

        (
            total_books,
            total_pages,
            mean_pages,
            total_available_books,
            total_borrowed_books,
        )
    }
}

/// Creates sample books used when the program starts.
/// # Returns
/// A vector containing the initial books for the library.
pub(crate) fn sample_books() -> Vec<Book> {
    vec![
        Book::new(
            "The Rust Programming Language, 3rd Edition".to_string(),
            "Carol Nichols".to_string(),
            2026,
            624,
            Genre::Computing,
            Status::Borrowed,
        ),
        Book::new(
            "Rust for Rustaceans: Idiomatic Programming for Experienced Developers".to_string(),
            "Jon Gjengset".to_string(),
            2021,
            280,
            Genre::Computing,
            Status::Available,
        ),
    ]
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            LibraryError::BookNotFound => "Le livre n'existe pas dans la bibliothèque",
            LibraryError::AlreadyAvailable => "Le livre est déjà dans la bibliothèque",
            LibraryError::AlreadyBorrowed => "Le livre est déjà emprunté",
        };
        write!(f, "{}", status_str)
    }
}
