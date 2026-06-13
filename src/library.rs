mod book;
pub use book::{Book, Genre, Status};

pub(crate) struct Library {
    books: Vec<Book>,
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

    pub(crate) fn search_by_title(&self, title: &str) {
        //TODO - Find a way to not display inside this function.
        let title = title.trim().to_lowercase();
        let mut found = false;

        println!("Liste de livres trouvé: ");
        for book in &self.books {
            if book.title().to_lowercase().contains(&title) {
                println!("{}", book.title());
                found = true;
            }
        }

        if !found {
            println!("Aucun livre trouve.");
        }
    }

    pub(crate) fn borrow_book(&mut self,id: u16) {
        // TODO- - Find a way to not display inside this function.
        // TODO - Handle the case where the book is not found instead of panicking.
        let book_to_borrow: &mut Book = self.books().iter().find(|book| book.id() == id).expect("Book not found");

        match book_to_borrow.status() {
            Status::Available => {
                book_to_borrow.set_status(Status::Borrowed);
                println!("Vous avez emprunté le livre: {}", book_to_borrow.title());
            }
            Status::Borrowed => println!("Ce livre est déjà emprunté"),
            _ => println!("Erreur, aucun status trouvé pour ce livre"),
        }
    }

    pub(crate) fn return_book(&mut self, id: u16) {}

    //fn display_stats() -> (usize, usize) {}
}
