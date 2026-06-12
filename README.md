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

```mermaid
%%{init: {"theme": "dark"}}%%
classDiagram
class Library {
    <<Struct>>
    -Vec~Livre~ books
}

Library "1" *-- "0..*" Livre : contient
```

### Book

```mermaid
%%{init: {"theme": "dark"}}%%
classDiagram
class Livre {
    <<Struct>>
    -String titre
    -String auteur
    -u16 annee_publication
    -u32 nombre_pages
    -Genre genre
    -Statut statut
}

class Genre {
    <<Enumeration>>
    Roman
    Science
    Informatique
    Biographie
    Autre
}

class Statut {
    <<Enumeration>>
    Disponible
    Emprunte
}

Livre --> Genre : possede
Livre --> Statut : possede
```

### Trait Rust

Un trait Rust se represente comme une interface avec une relation de
realisation :

```mermaid
%%{init: {"theme": "dark"}}%%
classDiagram
class Empruntable {
    <<Trait>>
    +emprunter() Result
    +retourner()
}

class Livre {
    <<Struct>>
}

Livre ..|> Empruntable : implemente
```
