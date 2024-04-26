use std::collections::HashMap;
use std::fmt;

use log::warn;

use crate::*;
use crate::classes::{Element, Library};

pub struct Package {
    pub name: String,
    pub header: String,
    pub body: String,
    pub uses: Vec<String>,
}

impl Package {
    pub fn set_header(&mut self, filename: &String) {
        if !self.header.is_empty() {
            if self.header.eq(filename) {
                return;
            }
            warn!(
                "Package {} already has a header from {}, the one in {} will be ignored.",
                self.name, self.header, filename
            );
            return;
        }
        self.header = filename.clone();
    }
    pub fn set_body(&mut self, filename: &String) {
        if !self.body.is_empty() {
            if self.body.eq(filename) {
                return;
            }
            warn!(
                "Package {} already has a body from {}, the one in {} will be ignored.",
                self.name, self.body, filename
            );
            return;
        }
        self.body = filename.clone();
    }
    pub fn add_use(&mut self, name: &String) {
        self.uses.push(name.to_string());
    }
    pub fn extend_uses(&mut self, uses: Vec<String>) {
        self.uses.extend(uses.clone());
    }
    pub fn resolve(&self, library: &String, libraries: &HashMap<String, Library>) -> Vec<Element> {
        let mut ret = resolve_uses(&self.uses, library, libraries);
        // package itself
        ret.push(Element {
            library: library.clone(),
            filename: self.header.clone(),
            language: "vhdl".to_string(),
        });
        if !self.body.is_empty() && self.body != self.header {
            ret.push(Element {
                library: library.clone(),
                filename: self.body.clone(),
                language: "vhdl".to_string(),
            });
        }
        ret
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Package {} with header [{}]  and body [{}])",
            self.name, self.header, self.body
        )
            .expect("panic message package #60");
        if !self.uses.is_empty() {
            writeln!(f).expect("panic message package #61");
        }
        for value in &self.uses {
            writeln!(f, "        Use: {value}").expect("panic message package #62");
        }
        write!(f, "")
    }
}
