use std::collections::HashMap;
#[derive(Debug)]
enum Genre {
    ROMAN, 
    NOVEL, 
    DRAMA, 
    FANTASY, 
    FICTION, 
    BIOGRAPHY,
    POETRY
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String, 
    description: String,
    isbn: String,
    language: String,
    genre: Genre,
    pages: i32, 
    publisher: String, 
    publication_date: String,
}

impl Book {
    fn print(&self){
        println!("{:#?}", self);
    }

    fn build_book(title: String, author:String, description: String, isbn: String, language: String, genre: Genre, pages:i32, publisher: String, publication_date: String) -> Self{
        Self{
            title ,
            author,
            description,
            isbn,
            language,
            genre,
            pages,
            publisher,
            publication_date,
    
        }
    
    }
}



fn main() {

    let book = Book::build_book(String::from("War and Peace"), String::from("Leo Tolstoy"),
        String::from("War and Peace, historical novel by Leo Tolstoy, originally published as Voyna i mir in 1865–69. This panoramic study of early 19th-century Russian society, noted for its mastery of realistic detail and variety of psychological analysis, is generally regarded as a masterwork of Russian literature and one of the world’s greatest novels."),
        String::from("90 - 178 - 10 - 89"),
        String::from("Russian"),
        Genre::NOVEL,
        1225,
        String::from("The Russian Messenger"),
        String::from("1869") );

    // println!("Book {:#?}", book);
    // dbg!(&book);
    book.print(); 

    let mut map = HashMap::new();
    let key = book.isbn.clone();
    map.insert(String::from(&book.isbn), book);

   match map.get_mut(&key) {
    Some(book) => book.title = String::from("bla bla"),
    None => ()
   };

    println!("{:?}", map);
}
