fn main() {
    /////////////////////////////////////////
    // AUDIOBOOK
    /////////////////////////////////////////
    let audiobook = Media::Audiobook {
        title: String::from("Audiobook #1")
    };

    println!("\n--------------------------");
    print_media(audiobook);
    println!("--------------------------\n");

    /////////////////////////////////////////
    // MOVIE
    /////////////////////////////////////////
    let good_movie = Media::Movie {
        title: String::from("Good Movide"),
        director: String::from("Good Director)")
    };

    println!("\n--------------------------");
    print_media(good_movie);
    println!("--------------------------\n");

    /////////////////////////////////////////
    // BOOK
    /////////////////////////////////////////
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author")
    };

    println!("\n--------------------------");
    print_media(bad_book);
    println!("--------------------------\n");
}

#[derive(Debug)]
enum Media {    
    Book { title: String, author: String },
    Movie {title: String, director: String },
    Audiobook {title: String }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}