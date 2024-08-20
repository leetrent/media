fn main() {
    /////////////////////////////////////////
    // AUDIOBOOK
    /////////////////////////////////////////
    let audiobook = Media::Audiobook {
        title: String::from("Audiobook #1")
    };

    /////////////////////////////////////////
    // MOVIE
    /////////////////////////////////////////
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director")
    };

    /////////////////////////////////////////
    // BOOK
    /////////////////////////////////////////
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author")
    };

    /////////////////////////////////////////
    // DESCRIPTON
    /////////////////////////////////////////
    println!("\n--------------------------");
    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());
    println!("\n--------------------------");
}

/////////////////////////////////////////
// MEDIA DECLARATION
/////////////////////////////////////////
#[derive(Debug)]
enum Media {    
    Book { title: String, author: String },
    Movie {title: String, director: String },
    Audiobook {title: String }
}

/////////////////////////////////////////
// MEDIA IMPLEMENTATION
/////////////////////////////////////////
impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book {title, author } => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie {title, director } => {
                format!("Movie: {} {}", title, director)
            },
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}