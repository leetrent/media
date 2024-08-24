use std::collections::btree_map::Values;

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
    // PODCAST
    /////////////////////////////////////////
    let podcast = Media::Podcast(17);

    /////////////////////////////////////////
    // PLACEHOLDER
    /////////////////////////////////////////
    let placeholder = Media::Placeholder;

    /////////////////////////////////////////
    // CATALOG
    /////////////////////////////////////////
    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("");
    match catalog.get_by_index(4) {
        Some(value) => {
            println!("(match) Item: {:#?}", value);
        }
        None => {
            println!("(match) No value at index 0");
        }
    }
    println!("");
    if let Some(value) = catalog.get_by_index(99) {
        println!("Item (pattern match): {:#?}", value);
    } else {
        println!("(pattern match) No value at index 99");
    }
    println!("");

    /////////////////////////////////////////
    // DESCRIPTON
    /////////////////////////////////////////
    // println!("\n--------------------------");
    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());
    // println!("\n--------------------------");
}

/////////////////////////////////////////
// CATALOG DECLARATION BLOCK
/////////////////////////////////////////
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

/////////////////////////////////////////
// CATALOG IMPLEMENTATION BLOCK
/////////////////////////////////////////
impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }  
    }
}

/////////////////////////////////////////
#[derive(Debug)]
enum Media {    
    Book { title: String, author: String },
    Movie {title: String, director: String },
    Audiobook {title: String },
    Podcast(u32),
    Placeholder
}

/////////////////////////////////////////
// MEDIA IMPLEMENTATION BLOCK
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
            },
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            },
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}