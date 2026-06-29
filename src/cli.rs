use crate::library::{Book, Genre, LibraryError, Status};
use std::io::{Write, stdin, stdout};

pub(crate) enum UserChoices {
    AddBook,
    DisplayBooks,
    SearchByTitle,
    BorrowBook,
    ReturnBook,
    DisplayStats,
    Quit,
}

/// Displays the main menu options.
pub(crate) fn display_menu() {
    const MENU_OPTIONS: &str = r#"=== Gestionnaire de bibliothèque ===
    Entrer un nombre entre 1 et 7 pour sélectionner une option.
    1. Ajouter un livre
    2. Afficher les livres
    3. Rechercher un livre par titre
    4. Emprunter un livre
    5. Retourner un livre
    6. Afficher les statistiques de la bibliothèque
    7. Quitter"#;

    println!("{MENU_OPTIONS}");
}

/// Reads and validates the user's menu choice.
///
/// # Returns
/// The selected `UserChoices` variant.
pub(crate) fn get_user_choice() -> UserChoices {
    loop {
        let choice = read_input("Entrez votre choix: ");

        match choice.as_str() {
            "1" => return UserChoices::AddBook,
            "2" => return UserChoices::DisplayBooks,
            "3" => return UserChoices::SearchByTitle,
            "4" => return UserChoices::BorrowBook,
            "5" => return UserChoices::ReturnBook,
            "6" => return UserChoices::DisplayStats,
            "7" => return UserChoices::Quit,
            _ => {
                println!("Choix invalide, veuillez réessayer.");
            }
        }
    }
}
/// Reads a trimmed line from standard input.
///
/// # Arguments
/// * `message` - The prompt displayed before reading input.
///
/// # Returns
/// The user input without leading or trailing whitespace.
fn read_input(message: &str) -> String {
    print!("{message}");
    stdout().flush().expect("Impossible d'afficher le message");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Impossible de lire l'entrée utilisateur");

    input.trim().to_string()
}

/// Reads a `u16` from user input and repeats until the input is valid.
///
/// # Arguments
/// * `message` - The prompt displayed before reading input.
/// * `error_message` - The message displayed when parsing fails.
///
/// # Returns
/// A valid `u16` entered by the user.
fn read_u16(message: &str, error_message: &str) -> u16 {
    loop {
        let input = read_input(message);

        match input.parse() {
            Ok(number) => return number,
            Err(_) => println!("{error_message}"),
        }
    }
}

/// Prompts the user for all information required to create a book.
///
/// # Returns
/// A `Book` created from the user's input.
pub(crate) fn prompt_book() -> Book {
    let title = read_input("Le titre de votre livre: ");
    let author = read_input("Le nom de l'auteur: ");
    let publication_year = read_u16(
        "L'année de publication: ",
        "Veuillez entrer une année de publication valide.",
    );
    let page_count = read_u16(
        "Combien de pages dans votre livre: ",
        "Veuillez entrer un nombre de pages valide.",
    );
    let genre: Genre = prompt_genre();
    let status: Status = prompt_status();

    Book::new(title, author, publication_year, page_count, genre, status)
}

/// Prompts the user for the book genre.
///
/// # Returns
/// The selected `Genre`.
fn prompt_genre() -> Genre {
    const GENRE_SELECTION: &str = r#"Sélectionner le genre du livre:
        1. Roman
        2. Science
        3. Informatique
        4. Histoire
        5. Autre"#;

    loop {
        println!("{GENRE_SELECTION}");
        let genre = read_input("Quel est le genre du livre: ");

        match genre.as_str() {
            "1" => return Genre::Novel,
            "2" => return Genre::Science,
            "3" => return Genre::Computing,
            "4" => return Genre::Historical,
            "5" => return Genre::Other,
            _ => {
                println!("Choix invalide, veuillez réessayer.")
            }
        };
    }
}

/// Prompts the user for the book status.
///
/// # Returns
/// The selected `Status`.
fn prompt_status() -> Status {
    const STATUS_SELECTION: &str = r#"Sélectionner le statut du livre:
        1. Disponible
        2. Emprunté"#;

    loop {
        println!("{STATUS_SELECTION}");
        let status = read_input("Quel est le statut du livre: ");

        match status.as_str() {
            "1" => return Status::Available,
            "2" => return Status::Borrowed,
            _ => {
                println!("Choix invalide, veuillez réessayer.");
            }
        }
    }
}

/// Prompts the user for a title search.
///
/// # Returns
/// The title or partial title entered by the user.
pub(crate) fn prompt_book_title() -> String {
    read_input("Entrez le titre du livre à rechercher: ")
}

/// Prompts the user for a book ID.
///
/// # Returns
/// A valid book ID entered by the user.
pub(crate) fn prompt_book_id() -> u16 {
    read_u16("Entrez l'ID du livre: ", "ID invalide. Entrez un nombre.")
}

/// Displays all books in the library.
///
/// # Arguments
/// * `books` - The books to display.
pub(crate) fn display_books(books: &[Book]) {
    if books.is_empty() {
        println!("\nAucun livre disponible dans la bibliothèque.");
        return;
    }
    println!("\nListe des livres dans la bibliothèque:");
    for book in books {
        println!("\n{book}");
    }
}

/// Displays a success message after a book has been added.
/// # Arguments
/// * `book` - The book that was added.
pub(crate) fn display_added_book(book: &Book) {
    println!("\n{} a été ajouté à la bibliothèque avec succès !\n",book.title());
}

/// Displays the result of a title search.
/// # Arguments
/// * `books` - References to books matching the search.
pub(crate) fn display_searched_books(books: &[&Book]) {
    if books.is_empty() {
        println!("\nAucun livre n'a été trouvé dans la bibliothèque avec votre recherche.");
        return;
    }
    for book in books {
        println!("{book}");
    }
}

/// Displays the result of a borrow operation.
/// # Arguments
/// * `result` - The borrowed book or the error returned by the library.
pub(crate) fn display_borrow_result(result: Result<&Book, LibraryError>) {
    match result {
        Ok(book) => println!("Le livre {} a bien été emprunté", book.title()),
        Err(err) => println!("Erreur: {err}"),
    }
}

/// Displays the result of a return operation.
/// # Arguments
/// * `result` - The returned book or the error returned by the library.
pub(crate) fn display_return_result(result: Result<&Book, LibraryError>) {
    match result {
        Ok(book) => println!("Le livre {} a bien été retourné", book.title()),
        Err(err) => println!("Erreur: {err}"),
    }
}

/// Displays library statistics.
///
/// # Arguments
/// * `total_books` - The total number of books.
/// * `total_pages` - The total number of pages.
/// * `mean_pages` - The average number of pages per book.
/// * `total_available_books` - The number of available books.
/// * `total_borrowed_books` - The number of borrowed books.
pub(crate) fn display_stats(
    total_books: u16,
    total_pages: u16,
    mean_pages: u16,
    total_available_books: u16,
    total_borrowed_books: u16,
) {
    println!("\nStatistiques de la bibliothèque:");
    println!("Total de livres: {}", total_books);
    println!("Total de pages: {}", total_pages);
    println!("Nombre moyen de pages par livre: {}", mean_pages);
    println!("Total de livres disponibles: {}", total_available_books);
    println!("Total de livres empruntés: {}", total_borrowed_books);
}

/// Displays the message before leaving the application.
pub(crate) fn display_leaving_message() {
    println!("Au revoir!");
}
