use std::collections::HashMap;

use crate::*;
use crate::classes::{ConfigurationInstance, Element, Instance, Library};
use crate::resolve_uses;

pub struct Architecture {
    pub name: String,
    pub filename: String,
    pub uses: Vec<String>,
    pub instances: Vec<Instance>,
}

impl Architecture {
    pub fn add_use(&mut self, name: &String) {
        self.uses.push(name.to_string());
    }
    pub fn extend_uses(&mut self, uses: Vec<String>) {
        self.uses.extend(uses.clone());
    }
    pub fn resolve(
        &self,
        library: &String,
        libraries: &HashMap<String, Library>,
        configuration_instances: &HashMap<String, ConfigurationInstance>,
    ) -> Vec<Element> {
        let mut ret = resolve_uses(&self.uses, library, libraries);
        // instances of architecture
        ret.extend(resolve_instances(
            &self.instances,
            library,
            libraries,
            configuration_instances,
            &self.uses,
        ));
        // arch itself
        ret.push(Element {
            library: library.to_string(),
            filename: self.filename.to_string(),
            language: "vhdl".to_string(),
        });
        ret
    }
}

/*
impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Architecture {} [{}])", self.name, self.filename)
            .expect("panic message architecture #1");
        // if there are uses or instances, print newline
        if !self.uses.is_empty() || !self.instances.is_empty() {
            writeln!(f).expect("panic message architecture #2");
        }
        for value in &self.uses {
            writeln!(f, "        Use: {value}").expect("panic message architecture #3");
        }
        for inst in &self.instances {
            writeln!(f, "        {}", inst).expect("panic message architecture #4");
        }
        write!(f, "")
    }
}
*/