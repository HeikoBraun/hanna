use std::collections::HashMap;

use crate::*;
use crate::classes::{Element, Library};

pub struct Entity {
    pub name: String,
    pub filename: String,
    pub uses: Vec<String>,
}

impl Entity {
    /*
    pub fn add_use(&mut self, name: &String) {
        self.uses.push(name.to_string());
    }
    pub fn extend_uses(&mut self, uses: Vec<String>) {
        self.uses.extend(uses.clone());
    }*/
    pub fn resolve(&self, library: &String, libraries: &HashMap<String, Library>) -> Vec<Element> {
        let mut ret = resolve_uses(&self.uses, library, libraries);
        ret.push(Element {
            library: library.to_string(),
            filename: self.filename.to_string(),
            language: "vhdl".to_string(),
        });
        ret
    }
}

/*
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity {} [{}])", self.name, self.filename).expect("panic message entity #40");
        if !self.uses.is_empty() {
            writeln!(f).expect("panic message entity #41");
        }
        for value in &self.uses {
            writeln!(f, "        Use: {value}").expect("panic message entity #42");
        }
        write!(f, "")
    }
}
*/
