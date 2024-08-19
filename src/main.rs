fn main() {
    /////////////////////////////////////////
    // AUDIOBOOK
    /////////////////////////////////////////
    let audiobook = Media::Audiobook {
        title: String::from("Audiobook #1")
    };

    // println!("\n--------------------------");
    // print_media(audiobook);
    // println!("--------------------------\n");

    /////////////////////////////////////////
    // MOVIE
    /////////////////////////////////////////
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director")
    };

    // println!("\n--------------------------");
    // print_media(good_movie);
    // println!("--------------------------\n");

    /////////////////////////////////////////
    // BOOK
    /////////////////////////////////////////
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author")
    };

    // println!("\n--------------------------");
    // print_media(bad_book);
    // println!("--------------------------\n");

    /////////////////////////////////////////
    // DESCRIPTON
    /////////////////////////////////////////
    println!("\n--------------------------");
    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());
    println!("\n--------------------------");
}

#[derive(Debug)]
enum Media {    
    Book { title: String, author: String },
    Movie {title: String, director: String },
    Audiobook {title: String }
}

impl Media {
    fn description(&self) -> String{
        if let Media::Book {title, author} = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie {title, director} = self {
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media Description")
        }
    }
}

// fn print_media(media: Media) {
//     println!("{:#?}", media);
// }