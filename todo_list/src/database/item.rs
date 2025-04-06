use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Item {
    id: i32,
    name: String
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item {{ id: {}, name: {} }}", self.id, self.name)
    }
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