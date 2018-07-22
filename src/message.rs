use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    magic_number: u32,
}

impl Message {
    pub fn from(content: &str, magic_number: u32) -> Self {
        Self {
            content: String::from(content),
            magic_number,
        }
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        self.magic_number == other.magic_number
    }
}

impl Eq for Message {}

impl Ord for Message {
    fn cmp(&self, other: &Message) -> Ordering {
        self.magic_number.cmp(&other.magic_number)
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Message) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
