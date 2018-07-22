use std::collections::BTreeSet;
use std::str;

use message::Message;

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub messages: BTreeSet<Message>,
}

#[derive(Debug, Clone)]
pub struct Difference {
    pub messages: Vec<String>,
}

impl Difference {
    pub fn is_empty(&self) -> bool {
        return self.messages.is_empty();
    }
}

impl Snapshot {
    pub fn from(raw_memory: &[u8]) -> Self {
        Self {
            messages: messages(raw_memory),
        }
    }

    pub fn diff(&self, new: &Snapshot) -> Difference {
        let difference = new
            .messages
            .difference(&self.messages)
            .cloned()
            .collect::<Vec<Message>>()
            .iter()
            .map(|d| d.content.clone())
            .collect::<BTreeSet<String>>()
            .into_iter()
            .collect();

        Difference {
            messages: difference,
        }
    }
}

fn messages(raw_memory: &[u8]) -> BTreeSet<Message> {
    let msgs = entry_idexes(raw_memory, "Loot of")
        .iter()
        .map(|idx| {
            let entry = &entry(&raw_memory[*idx..]);
            Message::from(entry, magic_number(&raw_memory, *idx, entry))
        })
        .collect();
    msgs
}

fn magic_number(raw_memory: &[u8], idx: usize, entry: &str) -> u32 {
    raw_memory[idx - 15..entry.len() + idx]
        .iter()
        .fold(0, |sum, b| sum + *b as u32)
}

fn entry_idexes(raw_memory: &[u8], keyword: &str) -> Vec<usize> {
    let mut indexes = vec![];
    let keyword_bytes = keyword.as_bytes();
    for (idx, _byte) in raw_memory.iter().enumerate() {
        if *_byte == keyword_bytes[0] {
            if raw_memory[idx..].len() > keyword_bytes.len() {
                if raw_memory[idx..idx + keyword_bytes.len()] == *keyword_bytes {
                    indexes.push(idx);
                }
            }
        }
    }
    return indexes;
}

#[test]
fn test_loot_entries() {
    let raw_memory = "01234Loot of a sandcrawler: 4 gold coin".as_bytes();
    assert_eq!(5, entry_idexes(raw_memory, "Loot of")[0]);
}

fn entry(chunk: &[u8]) -> String {
    let _bytes = chunk
        .iter()
        .filter(|b| **b != '\0' as u8)
        .take_while(|b| **b < 127u8)
        .map(|b| *b)
        .collect::<Vec<u8>>();
    let entry = str::from_utf8(&_bytes).unwrap();
    if let Some(last_space_index) = entry.rfind(' ') {
        return String::from(entry[..last_space_index].trim());
    }
    String::from(entry)
}

#[test]
fn test_entry() {
    let msg = "Loot of a sandcrawler: 4 gold coin ".as_bytes();
    let mut raw_memory = vec![128u8, 128u8, 128u8];
    raw_memory.extend_from_slice(msg);
    raw_memory.extend_from_slice(&[128u8, 128u8, 128u8]);
    assert_eq!(
        "Loot of a sandcrawler: 4 gold coin",
        &entry(&raw_memory[3..])
    );
}

#[test]
fn test_entry_artifacts_on_end_of_text() {
    let msg = "Loot of a x: x xl".as_bytes();
    let mut raw_memory = vec![128u8, 128u8, 128u8];
    raw_memory.extend_from_slice(msg);
    raw_memory.extend_from_slice(&[128u8, 128u8, 128u8]);
    assert_eq!("Loot of a x: x", &entry(&raw_memory[3..]));
}
