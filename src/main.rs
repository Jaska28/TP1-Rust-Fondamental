mod cli;
mod library;

use crate::cli::{
    UserChoices, display_added_book, display_books, display_borrow_result, display_leaving_message,
    display_menu, display_return_result, display_searched_books, display_stats, get_user_choice,
    prompt_book, prompt_book_id, prompt_book_title,
};
use crate::library::{Library, sample_books};

fn main() {
    let mut library = Library::new(sample_books());

    loop {
        display_menu();
        let user_choice = get_user_choice();

        match user_choice {
            UserChoices::AddBook => {
                let added_book = library.add_book(prompt_book());

                display_added_book(added_book);
            }

            UserChoices::DisplayBooks => display_books(library.books()),

            UserChoices::SearchByTitle => {
                let title = prompt_book_title();
                let books_found = library.search_by_title(&title);
                display_searched_books(&books_found);
            }

            UserChoices::BorrowBook => display_borrow_result(library
                .borrow_book(prompt_book_id())),

            UserChoices::ReturnBook => display_return_result(library
                .return_book(prompt_book_id())),

            UserChoices::DisplayStats => {
                let (
                    total_book,
                    total_pages,
                    mean_pages,
                    total_available_books,
                    total_borrowed_books,
                ) = library.get_stats();

                display_stats(
                    total_book,
                    total_pages,
                    mean_pages,
                    total_available_books,
                    total_borrowed_books,
                );
            }

            UserChoices::Quit => {
                display_leaving_message();
                break;
            }
        }
    }
}
