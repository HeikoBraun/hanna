use std::collections::HashMap;
use std::fmt;

use log::error;

use crate::classes::{Element, Library};

pub struct ConfigurationInstance {
    pub library: String,
    pub label: String, // can also be "others" or "all"
    pub comp: String,
    pub typ: String, // [open, entity, configuration]
    pub component: String,
    pub uses: Vec<String>,
}

impl ConfigurationInstance {
    pub fn resolve(&self, library: &String, libraries: &HashMap<String, Library>) -> Vec<Element> {
        let lib_name = if self.library == "work" {
            library
        } else {
            &self.library
        };
        match libraries.get(lib_name) {
            None => {
                error!("Library '{}' is unknown", lib_name)
            }
            Some(l) => {
                return l.resolve(&self.component, libraries);
            }
        }
        Vec::new()
    }
}


impl fmt::Display for ConfigurationInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConfigurationInstance {} Library: {} Comp: {} Typ: {} Component: {}",
            self.label, self.library, self.comp, self.typ, self.component
        )
    }
}
