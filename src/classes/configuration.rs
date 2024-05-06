use std::collections::HashMap;
//use std::fmt;
use std::process::exit;

use log::error;

use crate::*;
use crate::classes::{ConfigurationInstance, Element, Library};

pub struct Configuration {
    pub library: String,
    pub name: String,
    pub entity: String,
    pub filename: String,
    pub architecture: String,
    pub uses: Vec<String>,
    pub instances: HashMap<String, ConfigurationInstance>,
}
/*
impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
*/

impl Configuration {
    /*
    pub fn new() -> Configuration {
        Configuration {
            library: "".to_string(),
            name: "".to_string(),
            entity: "".to_string(),
            filename: "".to_string(),
            architecture: "".to_string(),
            uses: Vec::new(),
            instances: HashMap::new(),
        }
    }
    
     */

    /*
    pub fn add_use(&mut self, name: &String) {
        self.uses.push(name.to_string());
    }
    pub fn extend_uses(&mut self, uses: Vec<String>) {
        self.uses.extend(uses.clone());
    }
    */
    pub fn resolve(&self, libraries: &HashMap<String, Library>) -> Vec<Element> {
        let mut ret = resolve_uses(&self.uses, &self.library, libraries);
        let lib = match libraries.get(&self.library) {
            None => {
                error!("library '{}' is unknown", &self.library);
                exit(1)
            }
            Some(lib) => lib,
        };
        match lib.designs.get(&self.entity) {
            None => {
                error!(
                    "Error: Can't resolve design {} in library {}!",
                    self.entity, self.library
                );
            }
            Some(des) => {
                ret.extend(des.resolve(
                    &self.library,
                    libraries,
                    self.architecture.clone(),
                    &self.instances,
                ));
            }
        }
        ret.push(Element {
            library: self.library.clone(),
            filename: self.filename.clone(),
            language: "vhdl".to_string(),
        });
        ret
    }
}

/*
impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Configuration {} of {} of {} [{}]",
            self.name, self.entity, self.architecture, self.filename
        )
            .expect("panic message configuration #10");
        if !self.uses.is_empty() || !self.instances.is_empty() {
            writeln!(f).expect("panic message configuration #11");
        }
        for value in &self.uses {
            writeln!(f, "        Use: {value}").expect("panic message configuration #12");
        }
        for inst in self.instances.values() {
            writeln!(f, "        {}", inst).expect("panic message configuration #13");
        }
        write!(f, "")
    }
}
*/