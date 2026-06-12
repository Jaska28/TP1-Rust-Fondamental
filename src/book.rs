struct Book {
    id: u16,
    title: String,
    author: String,
    publication_year: u16,
    page_count: u16,
    genre: Genre,
    status: Status,
}

enum Genre {
    Fiction,
    Science,
    Computing,
    Biography,
    Other,
}

enum Status {
    Available,
    Borrowed,
}
