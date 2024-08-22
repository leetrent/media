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

    //println!("{:#?}", catalog);
    // println!("\n{:#?}", catalog.items.get(0));
    // println!("\n{:#?}", catalog.items.get(100));

    println!("\n\n");
    match catalog.items.get(0) {
        Option::Some(value) => {
            println!("Item: {:#?}", value)
        },
        Option::None => {
            println!("Nothing found at that index")
        }
    }
    println!("\n\n");
    match catalog.items.get(100) {
        Option::Some(value) => {
            println!("Item: {:#?}", value)
        },
        Option::None => {
            println!("Nothing found at that index")
        }
    }
    println!("\n\n");

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
}

/////////////////////////////////////////
// MEDIA DECLARATION BLOCK
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