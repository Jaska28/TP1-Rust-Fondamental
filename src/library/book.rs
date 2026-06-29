use std::fmt;

/// Represents a book in the library.
pub struct Book {
    id: u16,
    title: String,
    author: String,
    publication_year: u16,
    page_count: u16,
    genre: Genre,
    status: Status,
}

/// Represents the genre of a book.
pub enum Genre {
    Novel,
    Science,
    Computing,
    Historical,
    Other,
}

/// Represents whether a book is available or currently borrowed.
pub enum Status {
    Available,
    Borrowed,
}

impl Book {
    /// Creates a new book with the provided information.
    /// # Arguments
    /// * `title` - The book title.
    /// * `author` - The book author.
    /// * `publication_year` - The year the book was published.
    /// * `page_count` - The number of pages in the book.
    /// * `genre` - The book genre.
    /// * `status` - The current book status.
    /// # Returns
    /// A new `Book` with an ID initialized to 0.
    // Constructor
    pub fn new(
        title: String,
        author: String,
        publication_year: u16,
        page_count: u16,
        genre: Genre,
        status: Status,
    ) -> Self {
        Self {
            id: 0,
            title,
            author,
            publication_year,
            page_count,
            genre,
            status,
        }
    }

    // Getters
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn page_count(&self) -> u16 {
        self.page_count
    }

    // Setters
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }
}

/// Defines title-based search behavior.
pub trait Searchable {
    /// Checks whether an item matches a title search.
    /// # Arguments
    /// * `title` - The title or partial title to search for.
    /// # Returns
    /// `true` if the item matches the search, otherwise `false`.
    fn matches_title(&self, title: &str) -> bool;
}

impl Searchable for Book {
    fn matches_title(&self, title: &str) -> bool {
        self.title().to_lowercase().contains(&title.to_lowercase())
    }
}

// To be able to display the status enum
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Status::Available => "Le livre est disponible",
            Status::Borrowed => "Le livre est emprunté",
        };
        write!(f, "{}", status_str)
    }
}

// To be able to display the genre enum
impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Genre::Novel => "Roman",
            Genre::Science => "Science",
            Genre::Computing => "Informatique",
            Genre::Historical => "Historique",
            Genre::Other => "Autre",
        };
        write!(f, "{}", status_str)
    }
}

// Similar of __str__ (Python) method for display purposes
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  id: [{}] \
            \n  Nommé: {} \
            \n  Écrit par: {} \
            \n  Publié en: {} \
            \n  Genre: {} \
            \n  Status: {}",
            self.id, self.title, self.author, self.publication_year, self.genre, self.status,
        )
    }
}
