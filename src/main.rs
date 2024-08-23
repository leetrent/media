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

    // match catalog.get_by_index(5) {
    //     MightHaveAValue::ThereIsAValue(value) => {
    //         println!("(match) Item: {:#?}", value);
    //     }
    //     MightHaveAValue::NoValueAvailable => {
    //         println!("(match) No value at index 0");
    //     }
    // }

    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(10) {
        println!("Item (pattern match): {:#?}", value);
    } else {
        println!("(pattern match) No value at index 0");
    }

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

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            MightHaveAValue::ThereIsAValue(&self.items[index])
        } else {
            MightHaveAValue::NoValueAvailable
        }  
    }
}

enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable
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