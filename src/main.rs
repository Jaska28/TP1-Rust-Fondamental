mod library;
mod menu;

use crate::library::Library;
use crate::menu::{UserChoices, display_books, display_menu, get_user_choice, prompt_book};

// TODO - Check if there is slice I could replace in parameters instead of Vec<Book> and Vec<&Book> in the library struct and display_books function.

fn main() {
    let mut library = Library::new(Vec::new());

    loop {
        display_menu();
        let user_choice = get_user_choice();

        match user_choice {
            UserChoices::AddBook => library.add_book(prompt_book()),
            UserChoices::DisplayBooks => display_books(library.books()),
            UserChoices::SearchByTitle => {
                let title = "Jules"; // TODO - Prompt the user for the title to search
                library.search_by_title(title);
            }
            UserChoices::BorrowBook => {
                // TODO - Prompt user for book id
                library.borrow_book(1);
            }
            UserChoices::ReturnBook => {
                // TODO - Prompt user for book id
                library.return_book(1);
            }
            UserChoices::DisplayStats => println!("TODO - Afficher les statistiques."),
            UserChoices::Quit => {
                println!("Au revoir!");
                break;
            }
        }
    }
}
