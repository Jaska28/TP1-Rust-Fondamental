use crate::library::{Book, Genre, Status};
use std::io::stdin;
// TODO - Check a way to implement the result in the user input read_line
pub(crate) enum UserChoices {
    AddBook,
    DisplayBooks,
    SearchByTitle,
    BorrowBook,
    ReturnBook,
    DisplayStats,
    Quit,
}

pub(crate) fn display_menu() {
    // Check the method r# # ?
    const FILTER_SELECTION: &str = "Entrer un nombre entre 1 et 6 pour sélectionner une option.\
        \n1. Ajouter un livre\
        \n2. Afficher les livres\
        \n3. Rechercher un livre par titre\
        \n4. Emprunter un livre\
        \n5. Retourner un livre\
        \n6. Afficher les statistiques de la bibliothèque\
        \n10. Quitter";

    println!("{FILTER_SELECTION}");
}

pub(crate) fn get_user_choice() -> UserChoices {
    let choice = read_input("Entrez votre choix: ");

    match choice.as_str() {
        "1" => UserChoices::AddBook,
        "2" => UserChoices::DisplayBooks,
        "3" => UserChoices::SearchByTitle,
        "4" => UserChoices::BorrowBook,
        "5" => UserChoices::ReturnBook,
        "6" => UserChoices::DisplayStats,
        "10" => UserChoices::Quit,
        _ => {
            println!("Choix invalide, veuillez réessayer.");
            get_user_choice()
        }
    }
}
fn read_input(message: &str) -> String {
    println!("{message}");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

pub(crate) fn prompt_book() -> Book {
    // TODO - Add an equivalent of autoincrement id
    let title = read_input("Le titre de votre livre: ");
    let author = read_input("Le nom de l'auteur: ");
    let publication_year = read_input("L'année de publication: ");
    let page_count = read_input("Combien de pages à votre livre: ");
    //TODO - Display the genre choices and loop until choice is good or put a list of genre choices
    let genre = read_input("Quel est le genre du livre: ");

    let genre: Genre = Genre::Science;

    Book::new(1, title, author, 1990, 200, genre, Status::Available)
}

pub(crate) fn display_books(books: &Vec<Book>) {
    //TODO - display a message if there is no books.
    for book in books {
        println!("{}", book.title());
    }
}
