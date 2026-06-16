mod book;
use crate::library::BorrowError::AlreadyBorrowed;
pub use book::{Book, Genre, Status};

pub(crate) struct Library {
    books: Vec<Book>,
}

#[derive(Debug)]
pub enum BorrowError {
    BookNotFound,
    AlreadyBorrowed,
}

impl Library {
    // Constructor
    pub fn new(books: Vec<Book>) -> Self {
        Self { books }
    }

    // Getter
    pub fn books(&self) -> &Vec<Book> {
        &self.books
    }

    // Methods
    pub(crate) fn add_book(&mut self, book: Book) {
        (&mut self.books).push(book);
    }

    /// Search for books by title. The search is case-insensitive and will return all books that
    /// contain the search term in their title.
    /// # Arguments
    /// * `title` - The title or part of the title to search for.
    /// # Returns
    /// A vector of references to the books that match the search criteria.
    pub(crate) fn search_by_title(&self, title: &str) -> Vec<&Book> {
        let title = title.trim().to_lowercase();
        let mut found_books: Vec<&Book> = Vec::new();

        for book in &self.books {
            if book.title().to_lowercase().contains(&title) {
                found_books.push(book);
            }
        }
        found_books
    }

    /// Borrow a book by its ID. If the book is available, it will be marked as borrowed and a
    /// reference to the book will be returned. If the book is already borrowed or not found,
    /// an error will be returned.
    /// # Arguments
    /// * `id` - The ID of the book to borrow.
    /// # Returns
    /// A result containing a reference to the borrowed book if successful, or a `BorrowError` if
    /// the book is not found or already borrowed.
    pub(crate) fn borrow_book(&mut self, id: u16) -> Result<&Book, BorrowError> {
        let book_to_borrow: &mut Book = self
            .books
            .iter_mut()
            .find(|book| book.id() == id)
            .ok_or(BorrowError::BookNotFound)?;

        match book_to_borrow.status() {
            Status::Available => {
                book_to_borrow.set_status(Status::Borrowed);
                Ok(book_to_borrow)
            }
            Status::Borrowed => Err(AlreadyBorrowed),
        }
    }

    /// Return a book by its ID. If the book is currently borrowed, it will be marked as available
    /// and the function will return `true`. If the book is not found or was not borrowed
    /// , the function will return `false`.
    /// # Arguments
    /// * `id` - The ID of the book to return.
    /// # Returns
    /// `true` if the book was successfully returned, or `false` if the book was not found
    /// or was not borrowed.
    pub(crate) fn return_book(&mut self, id: u16) -> bool {
        let result = self
            .books
            .iter_mut()
            .find(|book| book.id() == id);

        let book_to_return = match result {
            Some(book) => book,
            None => return false,
        };

        match book_to_return.status() {
            Status::Borrowed => {
                book_to_return.set_status(Status::Available);
                true
            }
            Status::Available => false, // Book was not borrowed
        }
    }

    //fn display_stats() -> (usize, usize) {}
}
