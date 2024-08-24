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

    ///////////////////////////////////////////////////
    // UNWRAP
    ///////////////////////////////////////////////////
    let item = catalog.get_by_index(1);
    println!();
    println!("{:#?}", item.unwrap());
    println!();

    ///////////////////////////////////////////////////
    // UNWRAP (will panic)
    ///////////////////////////////////////////////////
    let item = catalog.get_by_index(99);
    println!();
    println!("{:#?}", item.unwrap());
    println!();




}

fn unwap() {
    
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