fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Audiobook #1")
    };

    println!("\n--------------------------");
    print_media(audiobook);
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