use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref PLULAR_SUFIXES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("ches" , "ch");
        m.insert("shes", "sh");
        m.insert("ies", "y");
        m.insert("ves", "fe");
        m.insert("oes", "o");
        m.insert("zes", "z");
        m.insert("s", "");
        m
    };
}

pub struct Loot {
    monster: String,
    items: Vec<String>
}

impl Loot {
    pub fn from(message: &str) -> Self {
        let msg_parts = message.split("Loot of a")
            .collect::<Vec<&str>>().into_iter().skip(1)
            .collect::<Vec<&str>>()[0]
            .split(":").collect::<Vec<&str>>();
        
        let monster = msg_parts[0].trim();
        let items = msg_parts[1].split(",")
            .collect::<Vec<&str>>().into_iter()
            .map(|s| s.trim()).collect::<Vec<&str>>()
            .into_iter().map(|s| String::from(s)).collect();
        Loot {
            monster: String::from(monster),
            items
        }
    }

    pub fn filter(mut self, loot_list: &[String]) -> Self {
        self.items = self.items.iter()
            .map(|i| i.to_owned().replace("a ", "").replace("an ", ""))
            .filter(|i| {
                loot_list.iter().any(|l| l == i) || {
                    let count_replacement = Regex::new(r"\d+ ").unwrap();
                    let trimmed = count_replacement.replace_all(i, "");
                    PLULAR_SUFIXES.iter()
                        .any(|(k, v)| {
                            loot_list.iter().any(|l| l == 
                            &format!("{}{}", Regex::new(&format!("(?P<w>.*){}", k)).unwrap().replace_all(&trimmed, "$w"), v))
                        })
                }

            })
            .collect::<Vec<String>>();
        self
    }

    pub fn monster_name<'a>(&'a self) -> &'a str {
        &self.monster
    }

    pub fn looted_items<'a>(&'a self) -> String {
        self.items.join(", ")
    }
}

#[test]
fn test_items() {
    let msg = "Loot of a x: y";
    let loot = Loot::from(msg);
    assert_eq!(&loot.monster, &"x");
    assert_eq!(&loot.items, &["y"]);
}


#[test]
fn test_multi_items() {
    let msg = "Loot of a x: y, z";
    let loot = Loot::from(msg);
    assert_eq!(&loot.monster, &"x");
    assert_eq!(&loot.items, &["y", "z"]);
}

#[test]
fn test_filter_should_omit_items_not_visible_in_filter() {
    let msg = "Loot of a x: y";
    let loot = Loot::from(msg).filter(&vec![String::from("x")]);
    assert!(loot.items.is_empty());
}

#[test]
fn test_filter_should_pass_items_visible_in_filter() {
    let msg = "Loot of a x: y";
    let loot = Loot::from(msg).filter(&vec![String::from("y")]);
    assert_eq!(&loot.items, &["y"]);;
}


#[test]
fn test_filter_should_omit_prefixes() {
    let msg = "Loot of a x: a y, an z, a a";
    let loot = Loot::from(msg).filter(&vec![String::from("y"), String::from("z"), String::from("a")]);
    assert_eq!(&loot.items, &["y", "z", "a"]);;
}

#[test]
fn test_filter_plular_ones() {
    let msg = "Loot of a x: 2 as";
    let loot = Loot::from(msg).filter(&vec![String::from("a")]);
    assert_eq!(&loot.items, &["2 as"]);;
}

#[test]
fn test_filter_plular_ones_while_received_many_msgs() {
    let msg = "Loot of a x: 2 as, 4 bs, 6 zches";
    let loot = Loot::from(msg).filter(&vec![String::from("a"), String::from("b"), String::from("zch")]);
    assert_eq!(&loot.items, &["2 as", "4 bs", "6 zches"]);;
}