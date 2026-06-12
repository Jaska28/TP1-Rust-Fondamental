# TP1 - Rust fondamental - Library Manager

## Functionalities
1. Display main menu
2. Display all books
3. Add a book
4. Search a book by title
5. Modifie the status of a book
6. Display stats

## Class diagrams
### Library
````plantuml
@startuml

!theme cyborg
skinparam classAttributeIconSize 0

struct Library {
    - books: struct book
}


@enduml
````

### Book
````plantuml
@startuml

!theme cyborg     
skinparam classAttributeIconSize 0 

struct Livre {
  - titre: String
  - auteur: String
  - annee_publication: u16
  - nombre_pages: u32
  - genre: Genre
  - statut: Statut
}

enum Genre {
  Roman
  Science
  Informatique
  Biographie
  Autre
}

enum Statut {
  Disponible
  Emprunte
}

Livre --> Genre : possède
Livre --> Statut : possède

@enduml
````
