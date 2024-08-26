use super::media::Media;

/////////////////////////////////////////
// CATALOG DECLARATION BLOCK
/////////////////////////////////////////
#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>
}

/////////////////////////////////////////
// CATALOG IMPLEMENTATION BLOCK
/////////////////////////////////////////
impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }  
    }
}