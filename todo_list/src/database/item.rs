#[derive(Debug)]
pub struct Item {
    id: i32,
    name: String
}

impl Item {
    pub fn new(id: i32, name: String) -> Self {
        Item { id, name }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}