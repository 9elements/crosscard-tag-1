#![allow(unused)]

//! Aufgabe 1:
//!
//! Dieses CMS soll ein content block System benutzen wie Notion eines hat. Einen
//! Typen für die Blöcke gibt es schon. Aber so richtig gut sieht der noch
//! nicht aus.
//!
//! Aufgabe
//!
//! Nutze dein Wissen über enums um das untere Beispiel zu verbessern.

/// A block of content
struct Block {
    /// This can be used with either image or text blocks
    title: Option<String>,
    /// Only use this if `is_image_block` is set to false
    content: Option<String>,
    is_image_block: bool,
    /// Only use this if `is_image_block` is set to true
    image_url: Option<String>,
}

fn main() {
    println!("Hello world!")
}

// Aufgabe 2:
//
// Das sieht doch schon viel besser aus! Aber das war noch lange nicht das Ende.
//
// Aufgabe
//
// Ändere den Typen des Titels so, dass zusätzlich die
// Dokumentenebene (1, 2, 3, 4, ...) angegeben werden kann.
//
// Ändere den Typen für Bilder damit auch alt-Attribute angegeben werden
// koennen.

// Aufgabe 3:
//
// Nun gut, bisher waren diese Typen ziemlich abstrakt. Lasst uns ein bisschen
// konkreter werden.
//
// Aufgabe 4:
//
// Erstelle eine "Page" Struktur in der der Titel (<title>) einer Seite gespeichert
// werden kann. Diese Struktur sollte auch die Blöcke der Seite gespeichert werden.
//
// Generiere das entsprechende HTML aus einer beliebigen Seite.
// Nutze dazu am besten einen trait (ToHtml).
//
//
// /// Diese Funktion sollte das endsprechende HTML ausgeben.
// fn print_html<T: ToHtml>(page: T) {
//     let rendered = page.to_html();
//     println!("{rendered}");
// }
