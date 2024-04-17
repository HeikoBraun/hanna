use crate::classes::{ConfigurationInstance, Element, Library};
use crate::Architecture;

use crate::Entity;
use log::{error, warn};
use std::collections::HashMap;
use std::fmt;

pub struct Design {
    pub library: String,
    pub name: String,
    pub entity: Entity,
    pub architectures: HashMap<String, Architecture>,
}
impl Design {
    pub fn set_entity(&mut self, ent: Entity) {
        if !self.entity.name.is_empty() {
            if self.entity.filename == ent.filename {
                return;
            }
            warn!(
                "Design {} already has an entity from {}, the one in {} will be ignored.",
                self.name, self.entity.filename, ent.filename
            );
            return;
        }
        self.entity = ent;
    }
    pub fn add_architecture(&mut self, name: String, arch: Architecture) {
        if let Some(old_arch) = self.architectures.get(&name) {
            if old_arch.filename == arch.filename {
                return;
            }
            warn!(
                "Design {} already has an architecture {} from {}, the one in {} will be ignored.",
                self.name, name, old_arch.filename, arch.filename
            );
            return;
        }
        self.architectures.insert(name, arch);
    }
    pub fn resolve(
        &self,
        library: &String,
        libraries: &HashMap<String, Library>,
        arch: String,
        configuration_instances: &HashMap<String, ConfigurationInstance>,
    ) -> Vec<Element> {
        let mut ret = self.entity.resolve(library, libraries);
        match self.architectures.get(&arch) {
            None => {
                error!(
                    "Error: Design {} doesn't have an architecture {}",
                    self.name, arch
                );
                return Vec::new();
            }
            Some(a) => {
                if self.entity.filename == a.filename {
                    // remove entity from list before inserting resolved architecture
                    ret.pop();
                }
                ret.extend(a.resolve(library, libraries, configuration_instances))
            }
        }
        ret
    }
}

impl fmt::Display for Design {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Design: {}", self.name).expect("panic message design #31");
        writeln!(f, "Entity: {}", self.entity).expect("panic message design #32");
        for arch in self.architectures.values() {
            writeln!(f, "Architecture: {}", arch).expect("panic message design #33");
        }
        write!(f, "")
    }
}
