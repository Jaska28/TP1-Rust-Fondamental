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

pub enum Genre {
    Fiction,
    Science,
    Computing,
    Biography,
    Other,
}

pub enum Status {
    Available,
    Borrowed,
}

impl Book {
    // Constructor
    pub fn new(
        id: u16,
        title: String,
        author: String,
        publication_year: u16,
        page_count: u16,
        genre: Genre,
        status: Status,
    ) -> Self {
        Self {
            id,
            title,
            author,
            publication_year,
            page_count,
            genre,
            status,
        }
    }

    // Getter
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    // Setter
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

// To be able to display the enum
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Status::Available => "Disponible",
            Status::Borrowed => "Emprunté",
        };
        write!(f, "{}", status_str)
    }
}

// Similar of __str__ (Python) method for display purposes
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} - {}", self.id(), self.title(), self.status,)
    }
}
