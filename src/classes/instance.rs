use crate::classes::{ConfigurationInstance, Element, Library, RE_ENT, RE_ENT2};
use log::{error, info, trace, warn};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::process::exit;

pub struct Instance {
    pub label: String,
    pub library: String,
    pub typ: String, // [component, entity, configuration]
    pub component: String,
}
impl Instance {
    pub fn resolve(
        &self,
        library: &String,
        libraries: &HashMap<String, Library>,
        configuration_instances: &HashMap<String, ConfigurationInstance>,
        uses: &Vec<String>,
    ) -> Vec<Element> {
        let re_use_2 = match Regex::new(r"(?imsx)^(?P<lib>\w+)\.(?P<part>\w+)$") {
            Ok(r) => r,
            Err(err) => {
                eprintln!("{}", err);
                exit(1)
            }
        };
        trace!(
            "Instance {} resolve ({:?}):",
            self.label,
            configuration_instances.keys()
        );
        // in configuration_instances?
        if let Some(ci) = configuration_instances.get(&self.label) {
            trace!("    in configuration_instances");
            return ci.resolve(library, libraries);
        }
        let all = "all@".to_owned() + &*self.component;
        if let Some(ci) = configuration_instances.get(&all) {
            trace!("    in configuration_instances");
            return ci.resolve(library, libraries);
        }

        let mut ret = Vec::new();
        let mut lib = &self.library;
        if lib == "work" {
            lib = library;
        }
        if self.typ == "entity" {
            // if component.contains("otp")
            match libraries.get(lib) {
                None => {
                    error!("Library '{}' is unknown", lib)
                }
                Some(l) => {
                    trace!("    entity");
                    ret.extend(l.resolve(&self.component, libraries))
                }
            }
        } else if self.typ == "configuration" {
            match libraries.get(lib) {
                None => {
                    error!("Library '{}' is unknown", lib)
                }
                Some(l) => {
                    trace!("    configuration");
                    ret.extend(l.resolve(&self.component, libraries))
                }
            }
        } else {
            // component
            let mut found = false;
            for (label, conf_inst) in configuration_instances {
                if (label == &self.label || label == "others" || label == "all")
                    && self.component == conf_inst.comp
                {
                    found = true;
                    match RE_ENT.captures(&conf_inst.component) {
                        None => {
                            error!("Error in Have: {} -> {}", label, conf_inst.to_string());
                        }
                        Some(caps) => {
                            let mut lib_names: Vec<String> = Vec::from([library.clone()]);
                            for usage in uses {
                                if let Some(caps_sub) = RE_ENT2.captures(usage) {
                                    let lib = caps_sub.name("lib").unwrap().as_str().to_string();
                                    if !lib_names.contains(&lib) {
                                        lib_names.push(lib);
                                    }
                                }
                            }
                            for lib_name in lib_names {
                                match libraries.get(&*lib_name) {
                                    None => {
                                        error!("Don't know library {}", lib_name)
                                    }
                                    Some(l) => {
                                        let e = &caps["entity"];
                                        if l.designs.contains_key(e)
                                            || l.configurations.contains_key(e)
                                            || l.modules.contains_key(e)
                                        {
                                            ret.extend(l.resolve(&self.component, libraries));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            }
            // if not resolved by a configuration
            if !found {
                let mut lib_names: Vec<String> = Vec::new();
                lib_names.push(library.to_string());
                for usage in uses {
                    for cap in re_use_2.captures_iter(usage) {
                        let lib = &cap["lib"];
                        let part = &cap["part"];
                        if lib != "work"
                            && lib != library
                            && !lib_names.contains(&lib.to_string())
                            && (part == "all" || part == self.component)
                        {
                            lib_names.push(lib.to_string())
                        }
                    }
                }
                for lib_name in lib_names {
                    if let Some(lib_tmp) = libraries.get(&*lib_name) {
                        if let Some(design) = lib_tmp.designs.get(&self.component) {
                            match design.architectures.len() {
                                0 => {
                                    warn!("Couldn't resolve component instance {}, because design {} has no architectures!",self.label,self.component);
                                }
                                1 => {
                                    found = true;
                                    if let Some(a) = design.architectures.values().next() {
                                        let mut design_name = self.component.clone();
                                        design_name.push('(');
                                        design_name.push_str(&a.name);
                                        design_name.push(')');
                                        ret.extend(lib_tmp.resolve(&design_name, libraries));
                                    }
                                    break;
                                }
                                _ => {
                                    let mut architectures: Vec<String> = Vec::new();
                                    for key in design.architectures.keys() {
                                        architectures.push(key.clone());
                                    }
                                    let architectures_str = architectures.join(", ");
                                    warn!("Couldn't resolve component instance {}, because design {} has several architectures ({})",self.label,self.component,architectures_str);
                                }
                            }
                        };
                    };
                }
            }
            if !found {
                trace!(
                    "Can't resolve {} in lib {} uses: {:?}",
                    self.to_string(),
                    library,
                    uses
                );
            }
            if !found {
                for (name, lib) in libraries {
                    if lib.modules.contains_key(&self.component) {
                        //warn!("{} can be resolved to a Verilog module in library {}.\nBut did you forget a 'use {}.all;' statement?",&self.component,name,name);
                        info!(
                            "{} can be resolved to a Verilog module in library {}.",
                            &self.component, name
                        );
                        ret.extend(lib.resolve(&self.component, libraries));
                        found = true;
                    }
                }
            }
            if !found {
                error!("Can't resolve {} in lib {}", self.to_string(), library);
            }
        }
        ret
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instance Label: {} Typ: {} Library: {} Component: {}",
            self.label, self.typ, self.library, self.component
        )
    }
}
