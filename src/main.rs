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
    // UNWRAP (found)
    ///////////////////////////////////////////////////
    //unwrap_item(&catalog, 1);

    ///////////////////////////////////////////////////
    // UNWRAP (will panic)
    ///////////////////////////////////////////////////
    //unwrap_item(&catalog, 99);

    ///////////////////////////////////////////////////
    // EXPECT (found)
    ///////////////////////////////////////////////////
    //expect_item(&catalog, 2);

    ///////////////////////////////////////////////////
    // EXPECT (will panic)
    ///////////////////////////////////////////////////
    //expect_item(&catalog, 99);

    ///////////////////////////////////////////////////
    // UNWRAP_OR(found)
    ///////////////////////////////////////////////////
    unwrap_or_item(&catalog, 3);

    ///////////////////////////////////////////////////
    // UNWRAP (will not panic)
    ///////////////////////////////////////////////////
    unwrap_or_item(&catalog, 99);
}

fn unwrap_item(catalog: &Catalog, index: usize) {
    let item = catalog.get_by_index(index);
    println!();
    println!("{:#?}", item.unwrap());
    println!();
}

fn expect_item(catalog: &Catalog, index: usize) {
    let item = catalog.get_by_index(index);
    println!();
    let msg = format!("Expected there to be an item at: {0}", index);
    println!("{:#?}", item.expect(&msg));
    println!();
}

fn unwrap_or_item(catalog: &Catalog, index: usize) {
    let placeholder = Media::Placeholder;
    let item = catalog.get_by_index(index);
    println!();
    println!("{:#?}", item.unwrap_or(&placeholder));
    println!();
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