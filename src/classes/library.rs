//use std::{fmt, fs};
use std::collections::{HashMap, HashSet};

use encoding::{DecoderTrap, Encoding};
use encoding::all::ISO_8859_1;
use glob::glob;
use log::{error, info, trace, warn};

use crate::*;
use crate::classes::Architecture;
use crate::classes::Configuration;
use crate::classes::Design;
use crate::classes::Entity;
use crate::classes::Package;
use crate::classes::RE_CONF_COMP_SPEC;
use crate::classes::RE_CONFIGURATION;
use crate::classes::RE_ENT;
use crate::classes::RE_ENT2;
use crate::classes::RE_INSTANCE;
use crate::classes::RE_LIBS;
use crate::classes::RE_MODULE;
use crate::classes::RE_PACKAGE;
use crate::classes::RE_PACKAGE_BODY;
use crate::classes::RE_USE;

pub struct Library {
    pub name: String,
    pub designs: HashMap<String, Design>,
    pub configurations: HashMap<String, Configuration>,
    pub packages: HashMap<String, Package>,
    pub modules: HashMap<String, String>,
    pub depends_on_libs: Vec<String>,
    pub ignore: bool,
    pub scopes: HashMap<String, Vec<String>>,
    pub all_elements: HashMap<String, Vec<Element>>,
}
/*
impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}
*/

impl Library {
    /*
    pub fn new() -> Library {
        Library {
            name: "".to_string(),
            designs: HashMap::new(),
            configurations: HashMap::new(),
            packages: HashMap::new(),
            modules: HashMap::new(),
            depends_on_libs: Vec::new(),
            ignore: false,
            scopes: Vec::new(),
            all_elements: Vec::new(),
        }
    }
    */

    /*
    pub fn print(&self) {
        println!("---------------------------------------------------");
        println!("Library {}", self.name);
        for key in self.designs.keys() {
            println!("    Design: {key}");
        }
        println!("---------------------------------------------------");
    }
    */

    /*
    pub fn has_design(&self, name: &String) -> bool {
        return self.designs.get(name).is_some();
    }
    */

    pub fn get_design(&mut self, name: &String) -> &mut Design {
        match self.designs.get_mut(name) {
            Some(_) => {}
            None => {
                let d: Design = Design {
                    library: self.name.to_string(),
                    name: name.to_string(),
                    entity: Entity {
                        name: String::from(""),
                        filename: String::from(""),
                        uses: Vec::new(),
                    },
                    architectures: HashMap::new(),
                };
                self.designs.insert(name.to_string(), d);
            }
        }
        self.designs.get_mut(name).unwrap() // Must exist now!
    }

    /*
    pub fn has_configuration(&self, name: &String) -> bool {
        return self.configurations.get(name).is_some();
    }

     */
    /*
    pub fn get_configuration(&mut self, name: &String) -> &mut Configuration {
        match self.configurations.get(name) {
            Some(_) => {}
            None => {
                let mut c: Configuration = Configuration::new();
                c.name = name.to_string();
                self.configurations.insert(name.to_string(), c);
            }
        }
        self.configurations.get_mut(name).unwrap() // Must exist now
    }

     */
    pub fn has_package(&self, name: &String) -> bool {
        return self.packages.get(name).is_some();
    }
    pub fn get_package(&mut self, name: &String) -> &mut Package {
        match self.packages.get(name) {
            Some(_) => {}
            None => {
                let p: Package = Package {
                    name: name.to_string(),
                    header: String::from(""),
                    body: String::from(""),
                    uses: Vec::new(),
                };
                self.packages.insert(name.to_string(), p);
            }
        }
        self.packages.get_mut(name).unwrap() // Must exist now!
    }

    pub fn analyze(&mut self) {
        if self.ignore {
            return;
        }
        info!("Analyzing library {} ....", self.name);
        for lang in KNOWN_LANGUAGES {
            let lang_s = String::from(lang);
            let mut used_filenames: HashSet<String> = HashSet::new();
            let mut found = false;
            let empty_scope: Vec<String> = Vec::new();
            let scope = self.scopes.get(&lang_s).unwrap_or(&empty_scope).clone();
            for pattern in scope {
                info!("Searching with glob pattern '{}' for {}", pattern, lang);
                for entry in glob(&pattern).expect("Failed to read glob pattern") {
                    match entry {
                        Ok(path) => {
                            if let Some(filename) = path.to_str() {
                                if !used_filenames.contains(filename) {
                                    used_filenames.insert(filename.to_string());

                                    self.all_elements.get_mut(&lang_s).unwrap_or(&mut Vec::new()).push(Element {
                                        library: self.name.clone(),
                                        filename: filename.to_string(),
                                        language: lang_s.clone(),
                                    });
                                    match lang {
                                        "vhdl" => self.analyze_vhdl_file(filename),
                                        "verilog" | "systemverilog" => self.analyze_verilog_file(filename),
                                        _ => {
                                            error!("142590864");
                                            exit(1);
                                        }
                                    }
                                    found = true;
                                } else {
                                    trace!("Ignoring duplicate glob entry {}", filename);
                                }
                            }
                        }
                        Err(e) => error!("{:?}", e),
                    }
                }
                if !found {
                    info!("    no files found!");
                }
            }
        }
    }

    pub fn read_file(&mut self, filename: &str) -> String {
        match fs::read(filename) {
            Ok(cont) => match ISO_8859_1.decode(&cont, DecoderTrap::Strict) {
                Ok(c) => c,
                Err(error) => {
                    error!("Error decoding file {}: {}", filename, error);
                    exit(1)
                }
            },
            Err(error) => {
                error!("Error opening file {}: {}", filename, error);
                exit(1)
            }
        }
    }

    pub fn analyze_vhdl_file(&mut self, filename: &str) {
        info!("Analyze {}", filename);
        // ToDo: Use RegexSet?
        let mut content = self.read_file(filename);
        // and remove unnecessary stuff
        content = pre_work_file_content(&content);
        // Uses, to add to all other found ones
        let mut uses: Vec<String> = Vec::new();
        let mut use_warned = false;
        for cap in RE_LIBS.captures_iter(&content) {
            let mut use_str = cap["name"].to_string();
            use_str.push_str(".all");
            uses.push(use_str);
            let lib_name = String::from(&cap["name"]);
            if lib_name == self.name {
                use_warned = true;
                warn!(
                    "{}: Don't use the name of the library ({}) itself, use 'work'!",
                    filename, self.name
                )
            } else if lib_name != "work" && !self.depends_on_libs.contains(&lib_name) {
                self.depends_on_libs.push(lib_name);
            }
        }
        for cap in RE_USE.captures_iter(&content) {
            uses.push(cap["content"].to_string());
            if cap["lib"] == self.name && !use_warned {
                warn!(
                    "{}: Don't use the name of the library ({}) itself, use 'work'!",
                    filename, self.name
                )
            }
        }
        // Entity
        for cap in RE_ENTITY.captures_iter(&content) {
            let design_name = &cap["name"];
            let d: &mut Design = self.get_design(&design_name.to_string());
            let e: Entity = Entity {
                name: design_name.to_string(),
                filename: filename.to_string(),
                uses: uses.clone(),
            };
            d.set_entity(e);
            trace!("Found entity {}", design_name.to_string());
        }
        for cap in RE_ARCHITECTURE.captures_iter(&content) {
            let design_name = &cap["entity"];
            let name = &cap["name"];
            let d: &mut Design = self.get_design(&design_name.to_string());
            let mut arch = Architecture {
                name: name.to_string(),
                filename: filename.to_string(),
                uses: uses.clone(), //.to_owned(),
                instances: Vec::new(),
            };
            trace!(
                "Found architecture {} of {}",
                name.to_string(),
                &design_name.to_string()
            );
            for cap_i in RE_INSTANCE.captures_iter(&cap["content"]) {
                let mut typ: String = "component".to_string();
                let mut component: String;
                let mut lib = String::from("work");
                component = cap_i.name("c_name").map_or("", |m| m.as_str()).to_string();
                if component.is_empty() {
                    typ = "entity".to_string();
                    lib = cap_i
                        .name("e_lib")
                        .map_or("work", |m| m.as_str())
                        .to_string();
                    component = cap_i.name("e_name").map_or("", |m| m.as_str()).to_string();
                }
                if component.is_empty() {
                    typ = "configuration".to_string();
                    lib = cap_i
                        .name("c_lib")
                        .map_or("work", |m| m.as_str())
                        .to_string();
                    component = cap_i
                        .name("con_name")
                        .map_or("", |m| m.as_str())
                        .to_string();
                }
                let inst = Instance {
                    label: cap_i["label"].to_string(),
                    library: lib,
                    typ,
                    component,
                };
                arch.instances.push(inst);
            }
            //
            d.add_architecture(name.to_string(), arch);
        }
        for cap in RE_CONFIGURATION.captures_iter(&content) {
            let design_name = &cap["entity"];
            let name = &cap["name"];
            let arch = &cap["arch"];
            let _d: &mut Design = self.get_design(&design_name.to_string());
            let mut conf = Configuration {
                library: self.name.clone(),
                name: name.to_string(),
                entity: design_name.to_string(),
                filename: filename.to_string(),
                architecture: arch.to_string(),
                uses: uses.clone(),
                instances: HashMap::new(),
            };
            trace!(
                "Found configuration {} of {} of {}",
                name.to_string(),
                arch.to_string(),
                &design_name.to_string()
            );
            for cap_i in RE_CONF_COMP_SPEC.captures_iter(&cap["content"]) {
                let label_decl: String = cap_i["label"].to_string();
                let comp: String = cap_i["comp"].to_string();
                let mut typ: String = "open".to_string();
                let mut component: String;
                let mut lib_name = self.name.clone();
                component = cap_i.name("open").map_or("", |m| m.as_str()).to_string();
                if component.is_empty() {
                    typ = "entity".to_string();
                    component = cap_i.name("entity").map_or("", |m| m.as_str()).to_string();
                }
                if component.is_empty() {
                    typ = "configuration".to_string();
                    component = cap_i.name("conf").map_or("", |m| m.as_str()).to_string();
                }
                if let Some(cap) = RE_ENT2.captures(&component) {
                    lib_name = cap["lib"].parse().unwrap();
                    component = cap["rest"].parse().unwrap();
                }
                for label_part in label_decl.split(',') {
                    let label = label_part.trim();
                    let inst = ConfigurationInstance {
                        library: lib_name.clone(),
                        label: label.to_string(),
                        comp: comp.clone(),
                        typ: typ.clone(),
                        component: component.clone(),
                        uses: uses.clone(),
                    };
                    let inst_name = if label == "all" {
                        "all@".to_owned() + &*comp
                    } else {
                        String::from(label)
                    };
                    conf.instances.insert(inst_name, inst);
                }
            }
            //
            if let Some(old_conf) = self.configurations.get(name) {
                if old_conf.filename != conf.filename {
                    warn!(
                    "Library {} already has a configuration {} from {}, the one in {} will be ignored.",
                    self.name, name, old_conf.filename, conf.filename
                )
                }
            } else {
                self.configurations.insert(name.to_string(), conf);
            }
        }
        for cap in RE_PACKAGE.captures_iter(&content) {
            let pkg_name = &cap["name"].to_string();
            let pkg: &mut Package;
            if self.has_package(pkg_name) {
                pkg = self.get_package(pkg_name);
                pkg.set_header(&filename.to_string());
                pkg.extend_uses(uses.clone());
            } else {
                let p = Package {
                    name: pkg_name.clone(),
                    header: filename.to_string(),
                    body: "".to_string(),
                    uses: uses.clone(),
                };
                self.packages.insert(pkg_name.clone(), p);
            }
            trace!("Found package {}", pkg_name);
        }
        for cap in RE_PACKAGE_BODY.captures_iter(&content) {
            let pkg_name = &cap["name"].to_string();
            if self.has_package(pkg_name) {
                let pkg = self.get_package(pkg_name);
                pkg.set_body(&filename.to_string());
                pkg.extend_uses(uses.clone());
            } else {
                let p = Package {
                    name: pkg_name.clone(),
                    header: "".to_string(),
                    body: filename.to_string(),
                    uses: uses.clone(),
                };
                self.packages.insert(pkg_name.clone(), p);
            }
            trace!("Found package body of {}", pkg_name);
        }
    }

    pub fn analyze_verilog_file(&mut self, filename: &str) {
        info!("Analyze {}", filename);
        let content = self.read_file(filename);
        for cap in RE_MODULE.captures_iter(&content) {
            let design_name = &cap["name"];
            self.modules
                .insert(design_name.to_string(), filename.to_string());
            trace!("Found module {}", design_name.to_string());
        }
    }

    pub fn get_design_names(&self) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        for key in self.designs.keys() {
            ret.push(key.clone())
        }
        ret.sort();
        ret
    }

    pub fn resolve(&self, name: &String, libraries: &HashMap<String, Library>) -> Vec<Element> {
        if self.ignore {
            return Vec::new();
        }
        let Some(caps) = RE_ENT.captures(name.as_str()) else {
            error!("No entity or configuration found in {}!", name);
            return Vec::new();
        };
        let entity = caps.name("entity").map_or("", |m| m.as_str()).to_string();
        let arch = caps.name("arch").map_or("", |m| m.as_str()).to_string();
        if arch.is_empty() {
            if let Some(conf) = self.configurations.get(&entity) {
                return conf.resolve(libraries);
            }
            if let Some(pack) = self.packages.get(&entity) {
                return pack.resolve(&self.name, libraries);
            }
            // test if there's a design with just one architecture!
            if let Some(design) = self.designs.get(&entity) {
                match design.architectures.len() {
                    0 => {
                        warn!(
                            "Couldn't resolve design {}, because it has no architectures!",
                            entity
                        );
                    }
                    1 => {
                        if let Some(a) = design.architectures.values().next() {
                            let mut design_name = entity;
                            design_name.push('(');
                            design_name.push_str(&a.name);
                            design_name.push(')');
                            return self.resolve(&design_name, libraries);
                        }
                    }
                    _ => {
                        let mut architectures: Vec<String> = Vec::new();
                        for key in design.architectures.keys() {
                            architectures.push(key.clone());
                        }
                        let architectures = architectures.join(", ");
                        warn!(
                            "Couldn't resolve design {}, because it has several architectures ({})",
                            entity, architectures
                        );
                    }
                }
            };
            if let Some(filename) = self.modules.get(&*entity) {
                info!("{} can be resolved to Verilog module!", entity,);
                let el = Element {
                    library: self.name.clone(),
                    filename: filename.clone(),
                    language: "verilog".to_string(),
                };
                return Vec::from([el]);
            }
            error!(
                "Can't resolve design {} in library {}! (No architecture given)",
                entity, self.name,
            );
            debug!(
                "Available designs in library {}:\n    {}",
                self.name,
                self.get_design_names().join("\n    ")
            );
            Vec::new()
        } else {
            match self.designs.get(&entity) {
                None => {
                    error!("Can't resolve design {} in library {}!", entity, self.name,);
                    debug!(
                        "Available designs in library {}:\n    {}",
                        self.name,
                        self.get_design_names().join("\n    ")
                    );
                    Vec::new()
                }
                Some(des) => des.resolve(&self.name, libraries, arch, &Default::default()),
            }
        }
    }

    pub fn list_designs(&self) -> Vec<String> {
        if self.ignore {
            return Vec::new();
        }
        let mut designs: Vec<String> = Vec::new();
        for design in self.designs.values() {
            for key in design.architectures.keys() {
                let mut design_name = design.name.to_string();
                design_name.push('(');
                design_name.push_str(key);
                design_name.push(')');
                designs.push(design_name);
            }
        }
        designs.sort();

        //ret.extend(self.configurations.keys());
        let mut configs: Vec<String> = Vec::new();
        for cfg in self.configurations.keys() {
            configs.push(cfg.clone());
        }
        configs.sort();
        designs.extend(configs);
        designs
    }
}

/*
impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-------------------------------").expect("panic message library #50");
        writeln!(f, "Library: {}", self.name).expect("panic message library #51");
        for design in self.designs.values() {
            writeln!(f, "{}", design).expect("panic message library #52");
        }
        for package in self.packages.values() {
            writeln!(f, "{}", package).expect("panic message library #53");
        }
        write!(f, "-------------------------------")
    }
}
*/
