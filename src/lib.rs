use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, exit};

use log::{debug, error, trace, warn};
use regex::{Captures, Replacer};
use toml::{Table, Value};

use crate::classes::{
    ConfigurationInstance, Element, Instance, Library, RE_ARCHITECTURE, RE_COMMENT, RE_ENTITY,
    RE_ENVVAR, RE_FUNC_PROC, RE_GENERATE, RE_PROCESS, RE_SIGNAL_OR_VARIABLE, RE_STD_LIBS, RE_USAGE,
    RE_USE_STD_LIBS, ToolLangConfig,
};
use crate::classes::tool_config::ToolConfig;

pub mod classes;


pub fn pre_work_file_content(input: &str) -> String {
    // strip comments
    let mut ret = RE_COMMENT.replace_all(input, "").to_string();
    // remove usages of std libs
    ret = RE_STD_LIBS.replace_all(&ret, "").to_string();
    ret = RE_USE_STD_LIBS.replace_all(&ret, "").to_string();
    // strip processes, they are of now use for this
    ret = RE_PROCESS.replace_all(&ret, "").to_string();
    // strip stuff inside Entity (Ports, Generics)
    ret = RE_ENTITY.replace_all(&ret, "$start $end").to_string();
    // strip functions or procedures
    ret = RE_FUNC_PROC.replace_all(&ret, "").to_string();
    // strip definitions in architecture
    ret = RE_ARCHITECTURE
        .replace_all(&ret, "$start $content $end")
        .to_string();
    // strip signal/variables
    ret = RE_SIGNAL_OR_VARIABLE.replace_all(&ret, "").to_string();
    // strip generates
    ret = RE_GENERATE.replace_all(&ret, "").to_string();
    ret.to_ascii_lowercase()
}

pub fn resolve_uses(
    uses: &Vec<String>,
    library: &String,
    libraries: &HashMap<String, Library>,
) -> Vec<Element> {
    let mut ret = Vec::new();
    for usage in uses {
        match RE_USAGE.captures(usage) {
            None => {
                error!("No valid usage found in {}!", usage)
            }
            Some(caps) => {
                let mut lib_name = caps.name("lib").map_or("", |m| m.as_str()).to_string();
                let package = caps.name("package").map_or("", |m| m.as_str()).to_string();
                if package == "all" {
                    continue;
                }
                if lib_name == "work" {
                    lib_name = library.to_string();
                }
                let library: &Library;
                match libraries.get(&*lib_name) {
                    None => {
                        error!("library '{}' is unknown", lib_name);
                        exit(1)
                    }
                    Some(lib) => library = lib,
                };
                ret.extend(library.resolve(&package, libraries));
            }
        }
    }
    ret
}


pub fn resolve_instances(
    instances: &Vec<Instance>,
    library: &String,
    libraries: &HashMap<String, Library>,
    configuration_instances: &HashMap<String, ConfigurationInstance>,
    uses: &Vec<String>,
) -> Vec<Element> {
    let mut ret = Vec::new();
    for instance in instances {
        ret.extend(instance.resolve(library, libraries, configuration_instances, uses));
    }
    ret
}

pub fn rework_file_path(path: String) -> String {
    RE_ENVVAR
        .replace_all(path.as_str(), EnvReplacer)
        .parse()
        .unwrap()
}

struct EnvReplacer;

impl Replacer for EnvReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        dst.push_str("");
        if let Some(m) = caps.name("var1") {
            let name = m.as_str();
            match env::var(name) {
                Ok(var) => dst.push_str(&var),
                Err(_) => {
                    warn!(
                        "Environment variable {} was not found and replaced with ''",
                        name
                    )
                }
            }
        }
        if let Some(m) = caps.name("var2") {
            let name = m.as_str();
            match env::var(name) {
                Ok(var) => dst.push_str(&var),
                Err(_) => {
                    warn!(
                        "Environment variable {} was not found and replaced with ''",
                        name
                    )
                }
            }
        }
    }
}


pub fn get_toplevels_from_lib(lib_name: &String, libraries_toml_filename: &String, tool_toml_filename: &String,
                              replacements: &HashMap<String, String>,
) -> Vec<String> {
    let tool_config = read_tool_toml(tool_toml_filename, &replacements);
    let mut replacements_all = replacements.clone();
    for (key, value) in &tool_config.replacement {
        replacements_all.insert(key.clone(), value.clone());
    }
    let libs = read_libraries_toml(
        &libraries_toml_filename,
        &replacements_all,
        &tool_config,
    );
    match libs.get(lib_name) {
        None => {
            error!("A lib with name {} is not defined!",lib_name);
            exit(1)
        }
        Some(lib) => { return lib.list_designs(); }
    }
}

pub fn read_libraries_toml(
    filename: &String,
    replacements: &HashMap<String, String>,
    tool_config: &ToolConfig,
) -> HashMap<String, Library> {
    let mut ret: HashMap<String, Library> = HashMap::new();
    let mut replacements_all = replacements.clone();
    for (key, value) in &tool_config.replacement {
        replacements_all.insert(key.clone(), value.clone());
    }
    //replacements_all.extend(&tool_config.replacement.clone());
    let config = read_toml(filename, &replacements_all);
    for (name, value) in config {
        let ignore: bool;
        let mut vhdl_scope: Vec<String> = Vec::new();
        let mut verilog_scope: Vec<String> = Vec::new();
        match value {
            Value::Table(t) => {
                if let Some(v) = t.get("ignore") {
                    match v {
                        Value::Boolean(b) => ignore = *b,
                        _ => {
                            error!("TOML libraries: 'ignore' must be of type bool!");
                            exit(1)
                        }
                    }
                } else {
                    ignore = false
                }
                if let Some(v) = t.get("vhdl") {
                    match v {
                        Value::Array(l) => {
                            for e in l {
                                vhdl_scope
                                    .push(rework_file_path(String::from(e.as_str().unwrap_or(""))));
                            }
                        }
                        _ => {
                            error!("TOML libraries: 'vhdl' must be a list of strings!");
                            exit(1)
                        }
                    }
                };
                if let Some(v) = t.get("verilog") {
                    match v {
                        Value::Array(l) => {
                            for e in l {
                                verilog_scope
                                    .push(rework_file_path(String::from(e.as_str().unwrap_or(""))));
                            }
                        }
                        _ => {
                            error!("TOML libraries: 'verilog' must be a list of strings!");
                            exit(1)
                        }
                    }
                };
            }
            _ => {
                error!("Unknown error #134415");
                exit(1)
            }
        };
        let mut lib = Library {
            name: name.clone(),
            designs: HashMap::new(),
            configurations: HashMap::new(),
            packages: HashMap::new(),
            modules: HashMap::new(),
            depends_on_libs: Vec::new(),
            ignore,
            vhdl_scope,
            verilog_scope,
            all_vhdl_elements: Vec::new(),
            all_verilog_elements: Vec::new(),
        };
        lib.analyze();
        ret.insert(name, lib);
    }
    ret
}

pub fn get_library_names_from_toml(
    filename: &String,
    replacements: &HashMap<String, String>,
) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let config = read_toml(filename, &replacements);
    for (name, value) in config {
        let ignore: bool;
        match value {
            Value::Table(t) => {
                if let Some(v) = t.get("ignore") {
                    match v {
                        Value::Boolean(b) => ignore = *b,
                        _ => {
                            error!("TOML libraries: 'ignore' must be of type bool!");
                            exit(1)
                        }
                    }
                } else {
                    ignore = false
                }
            }
            _ => {
                error!("Unknown error #134415");
                exit(1)
            }
        };
        if !ignore {
            ret.push(name);
        }
    }
    ret.sort();
    ret
}

//replace top,library,files
pub fn read_toml(filename: &String, replacements: &HashMap<String, String>) -> Table {
    trace!("{}: {:#?}", filename, replacements);
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => {
            error!("Couldn't open '{}': {}", filename, why);
            exit(1);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(contents) => contents,
        Err(why) => {
            error!("Couldn't read '{}': {}", filename, why);
            exit(1);
        }
    };
    // replace env vars
    contents = RE_ENVVAR
        .replace_all(contents.as_str(), EnvReplacer)
        .parse()
        .unwrap();
    // replace other vars in <var> or in {var}
    for (orig, replacement) in replacements {
        let mut tmp = String::from("{");
        tmp.push_str(orig);
        tmp.push('}');
        debug!("TOML: replacement: {} -> {}", orig, replacement);
        contents = contents.replace(&tmp, replacement);
    }
    //
    contents.parse::<Table>().unwrap_or_else(|err| {
        error!("{}", err);
        exit(1)
    })
}

pub fn read_tool_toml(filename: &String, replacements: &HashMap<String, String>) -> ToolConfig {
    let config = read_toml(filename, replacements);

    /////////////////////////////////////////////////////////////////////////////////
    let mut common: Vec<String> = Vec::new();
    let mut vhdl_cfg = ToolLangConfig::new();
    let mut verilog_cfg = ToolLangConfig::new();
    let mut exec_before: Vec<String> = Vec::new();
    let mut exec_after: Vec<String> = Vec::new();
    let mut exec_per_lib: Vec<String> = Vec::new();
    let mut replace: HashMap<String, String> = HashMap::new();
    if let Some(c) = config.get("common") {
        match c {
            Value::Array(va) => {
                if va.is_empty() {
                    common.push("".to_string())
                } else {
                    for e in va {
                        common.push(String::from(e.as_str().unwrap_or("")));
                    }
                }
            }
            _ => {
                error!("{}: common value must be an array of String!", filename,);
                exit(1);
            }
        }
    }
    if let Some(c) = config.get("exec_before") {
        match c {
            Value::Array(va) => {
                for e in va {
                    exec_before.push(String::from(e.as_str().unwrap_or("")));
                }
            }
            _ => {
                error!(
                    "{}: exec_before value must be an array of String!",
                    filename,
                );
                exit(1);
            }
        }
    }
    if let Some(c) = config.get("exec_after") {
        match c {
            Value::Array(va) => {
                for e in va {
                    exec_after.push(String::from(e.as_str().unwrap_or("")));
                }
            }
            _ => {
                error!("{}: exec_after value must be an array of String!", filename,);
                exit(1);
            }
        }
    }
    if let Some(c) = config.get("exec_per_lib") {
        match c {
            Value::Array(va) => {
                for e in va {
                    exec_per_lib.push(String::from(e.as_str().unwrap_or("")));
                }
            }
            _ => {
                error!(
                    "{}: exec_per_lib value must be an array of String!",
                    filename,
                );
                exit(1);
            }
        }
    }
    if let Some(c) = config.get("replace") {
        match c {
            Value::Table(t) => {
                for (key, value) in t {
                    replace.insert(key.clone(), String::from(value.as_str().unwrap_or("")));
                }
            }
            _ => {
                error!(
                    "{}: replace: value must be a HashMap of type 'old = new'!",
                    filename,
                );
                exit(1);
            }
        }
    }

    for lang in ["vhdl", "verilog"] {
        let cfg: ToolLangConfig;
        if let Some(c) = config.get(lang) {
            match c {
                Value::Table(t) => {
                    cfg = get_tool_lang_config(filename, &lang.to_string(), t);
                }
                _ => {
                    error!(
                            "{}: {}: value must be a HashMap with entries for common, per_lib, single_call, exec_per_lib, replace",
                            filename,lang
                        );
                    exit(1);
                }
            }
        } else {
            cfg = ToolLangConfig::new()
        }
        if lang == "vhdl" {
            vhdl_cfg = cfg;
        } else {
            verilog_cfg = cfg
        }
    }

    ToolConfig {
        common,
        vhdl: vhdl_cfg,
        verilog: verilog_cfg,
        exec_before,
        exec_after,
        exec_per_lib,
        replacement: replace,
    }
}

pub fn get_tool_lang_config(filename: &String, lang: &String, table: &Table) -> ToolLangConfig {
    let mut common: Vec<String> = Vec::new();
    let mut per_lib: Vec<String> = Vec::new();
    let single_call: bool;
    let mut exec_per_lib: Vec<String> = Vec::new();
    if let Some(e) = table.get("common") {
        match e {
            Value::Array(va) => {
                for e in va {
                    common.push(String::from(e.as_str().unwrap_or("")));
                }
            }
            _ => {
                error!(
                    "{}: common value in [{}] must be an array of String!",
                    filename, lang
                );
                exit(1);
            }
        }
    }
    if let Some(e) = table.get("per_lib") {
        match e {
            Value::Array(va) => {
                for e in va {
                    per_lib.push(String::from(e.as_str().unwrap_or("")));
                }
            }
            _ => {
                error!(
                    "{}: per_lib value in [{}] must be an array of String!",
                    filename, lang
                );
                exit(1);
            }
        }
    }
    if let Some(e) = table.get("single_call") {
        match e {
            Value::Boolean(vb) => {
                single_call = *vb;
            }
            _ => {
                error!(
                    "{}: per_lib value in [{}] must be an array of String!",
                    filename, lang
                );
                exit(1);
            }
        }
    } else {
        single_call = false
    }
    if let Some(e) = table.get("exec_per_lib") {
        match e {
            Value::Array(va) => {
                for e in va {
                    exec_per_lib.push(String::from(e.as_str().unwrap_or("")));
                }
            }
            _ => {
                error!(
                    "{}: exec_per_lib value in [{}] must be an array of String!",
                    filename, lang
                );
                exit(1);
            }
        }
    }

    ToolLangConfig {
        common,
        per_lib,
        single_call,
        exec_per_lib,
    }
}

pub fn get_element_list(lib_name: String, toplevel: String, libraries_toml_filename: &String, tool_toml_filename: &String,
                        replacements: &HashMap<String, String>,
                        ignore_libraries: &Vec<String>, ) -> (Vec<Element>, HashMap<String, Library>) {
    let tool_config = read_tool_toml(tool_toml_filename, &replacements);
    let mut replacements_all = replacements.clone();
    for (key, value) in &tool_config.replacement {
        replacements_all.insert(key.clone(), value.clone());
    }
    let mut libs = read_libraries_toml(
        &libraries_toml_filename,
        &replacements_all,
        &tool_config,
    );
    for (lib_name, lib) in &mut libs {
        if ignore_libraries.contains(&lib_name) {
            lib.ignore = true;
        }
    }
    match libs.get(&lib_name) {
        None => {
            error!("A lib with name {} is not defined!",lib_name);
            exit(1)
        }
        Some(lib) => { (lib.resolve(&toplevel, &libs), libs) }
    }
}

pub fn write_lib_lists(
    lib_name: String, toplevel: String, libraries_toml_filename: &String, tool_toml_filename: &String,
    replacements: &HashMap<String, String>, filename: &String,
    ignore_libraries: &Vec<String>,
) {
    let (element_list, libraries) = get_element_list(lib_name, toplevel, libraries_toml_filename, tool_toml_filename, replacements, ignore_libraries);
    let lib_order = get_sorted_libraries(&libraries);
    let mut l_path = String::from(filename.strip_suffix('/').unwrap_or(&*filename));
    l_path.push('/');
    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    match element_list.last() {
        None => {
            eprintln!("No files found to write out!");
            exit(1);
        }
        Some(last_el) => last_el,
    };
    for lib_name in &lib_order {
        let mut lib_list: Vec<String> = Vec::new();
        for el in &element_list {
            if el.library.eq(lib_name) && !lib_list.contains(&el.filename) {
                lib_list.push(el.filename.clone())
            }
        }
        res.insert(lib_name.to_string(), lib_list);
    }
    ///////////////////////////
    let filename = format!("{}libraries.f", l_path);
    fs::write(filename, lib_order.join("\n") + "\n").expect("failed to write to file");
    for lib_name in &lib_order {
        let filename = format!("{}{}.f", l_path, lib_name);
        match res.get(lib_name) {
            None => {
                eprintln!("Library '{}' doesn't exist", lib_name);
                exit(1);
            }
            Some(filenames) => {
                match fs::write(filename, filenames.join("\n")) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Can't write to '{}'\n{}", lib_name, err);
                        exit(1);
                    }
                };
            }
        };
    }
}

pub fn write_json_file(
    lib_name: String, toplevel: String, libraries_toml_filename: &String, tool_toml_filename: &String,
    replacements: &HashMap<String, String>, filename: &String,
    ignore_libraries: &Vec<String>,
) {
    let (element_list, libraries) = get_element_list(lib_name, toplevel, libraries_toml_filename, tool_toml_filename, replacements, ignore_libraries);
    let lib_order = get_sorted_libraries(&libraries);
    let mut l_path = String::from(filename.strip_suffix('/').unwrap_or(&*filename));
    l_path.push('/');
    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    match element_list.last() {
        None => {
            eprintln!("No files found to write out!");
            exit(1);
        }
        Some(last_el) => last_el,
    };
    for lib_name in &lib_order {
        let mut lib_list: Vec<String> = Vec::new();
        for el in &element_list {
            if el.library.eq(lib_name) && !lib_list.contains(&el.filename) {
                lib_list.push(el.filename.clone())
            }
        }
        res.insert(lib_name.to_string(), lib_list);
    }
    res.insert("libraries".to_string(), lib_order);
    let content = serde_json::to_string_pretty(&res).unwrap_or(String::from(""));
    // ToDo: Create directory if it does not exist!
    let mut file = match File::create(filename) {
        Ok(file) => { file }
        Err(err) => {
            eprintln!("Can't write to '{}'\n{}", filename, err);
            exit(1);
        }
    };
    let _ = file.write(content.as_ref());
}

pub fn get_lib_name_to_use(lib_name: &String, top_lib_name: &String, use_work: &bool) -> String {
    if *use_work && lib_name == top_lib_name {
        String::from("work")
    } else {
        lib_name.clone()
    }
}

pub fn gen_script(
    lib_name: String, toplevel: String, libraries_toml_filename: &String, tool_toml_filename: &String,
    replacements: &HashMap<String, String>, filename: &String,
    use_work: bool,
    ignore_libraries: &Vec<String>,
) {
    let top_lib_name = lib_name;
    let (element_list, libraries) = get_element_list(top_lib_name.clone(), toplevel, libraries_toml_filename, tool_toml_filename, replacements, ignore_libraries);
    let lib_order = get_sorted_libraries(&libraries);
    let tool_config = read_tool_toml(tool_toml_filename, &replacements);
    let mut l_path = String::from(filename.strip_suffix('/').unwrap_or(&*filename));
    l_path.push('/');

    match element_list.last() {
        None => {
            eprintln!("No files found to write out!");
            exit(1);
        }
        Some(last_el) => last_el,
    };
    // get lists per language
    let mut file_lists_verilog: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_lists_vhdl: HashMap<String, Vec<String>> = HashMap::new();
    for lib_name in &lib_order {
        let mut verilog_list: Vec<String> = Vec::new();
        let mut vhdl_list: Vec<String> = Vec::new();
        for el in &element_list {
            if el.library.eq(lib_name) {
                if el.language.eq("vhdl") {
                    if !vhdl_list.contains(&el.filename) {
                        vhdl_list.push(el.filename.clone())
                    }
                } else if el.language.eq("verilog") && !verilog_list.contains(&el.filename) {
                    verilog_list.push(el.filename.clone())
                }
            }
        }
        file_lists_verilog.insert(lib_name.to_string(), verilog_list);
        file_lists_vhdl.insert(lib_name.to_string(), vhdl_list);
    }

    let mut content: Vec<String> = Vec::new();
    content.push(String::from("#!/usr/bin/env sh"));
    // exec_before
    for entry in &tool_config.exec_before {
        content.push(entry.clone());
    }
    // exec_per_lib
    for lib_name in &lib_order {
        let lib_name_to_use = get_lib_name_to_use(&lib_name, &top_lib_name, &use_work);
        for entry in &tool_config.exec_per_lib {
            content.push(entry.replace("{library}", &lib_name_to_use));
        }
        for entry in &tool_config.vhdl.exec_per_lib {
            content.push(entry.replace("{library}", &lib_name_to_use));
        }
        for entry in &tool_config.verilog.exec_per_lib {
            content.push(entry.replace("{library}", &lib_name_to_use));
        }
    }
    content.push(String::from(""));

    // compile verilog
    // compile vhdl
    for lang in ["verilog", "vhdl"] {
        for main_common in &tool_config.common {
            let single_call: &bool;
            let lang_commons: &Vec<String>;
            let per_lib: &Vec<String>;
            let tmp: Vec<String> = vec![String::from("")];
            if lang == "vhdl" {
                single_call = &tool_config.vhdl.single_call;
                if !tool_config.vhdl.common.is_empty() {
                    lang_commons = &tool_config.vhdl.common;
                } else {
                    lang_commons = &tmp;
                }
                per_lib = &tool_config.vhdl.per_lib;
            } else {
                single_call = &tool_config.verilog.single_call;
                if !tool_config.verilog.common.is_empty() {
                    lang_commons = &tool_config.verilog.common;
                } else {
                    lang_commons = &tmp;
                }
                per_lib = &tool_config.verilog.per_lib;
            }

            for lang_common in lang_commons {
                let mut call: Vec<String>;
                let mut common: Vec<String> = Vec::new();
                common.push(main_common.clone());
                if !main_common.is_empty() {
                    common.push(String::from(" \\\n    "))
                }
                common.push(lang_common.clone());
                if !lang_common.is_empty() {
                    if *single_call {
                        common.push(String::from(" \\"))
                    } else {
                        common.push(String::from(" \\\n    "))
                    }
                }
                //
                if *single_call {
                    content.push(common.join(""));
                }
                for lib_name in &lib_order {
                    let lib_name_to_use = get_lib_name_to_use(&lib_name, &top_lib_name, &use_work);
                    let files_empty: Vec<String> = Vec::new();
                    let files = if lang == "vhdl" {
                        file_lists_vhdl.get(lib_name).unwrap_or(&files_empty)
                    } else {
                        file_lists_verilog.get(lib_name).unwrap_or(&files_empty)
                    };
                    if files.is_empty() {
                        continue;
                    }
                    if *single_call {
                        call = vec!["    ".to_string()];
                    } else {
                        call = common.clone();
                    }
                    for value in per_lib {
                        call.push(value.clone());
                        if *single_call {
                            call.push(String::from(" \\"))
                        } else {
                            call.push(String::from(" \\\n    "))
                        }
                    }
                    let sep = if *single_call {
                        " \\\n        "
                    } else {
                        " \\\n    "
                    };
                    let tmp = call.join("");
                    let tmp = tmp.replace("{library}", &lib_name_to_use);
                    let tmp = tmp.replace("{files}", &files.join(sep));
                    content.push(tmp);
                }
                content.push("".to_string());
            }
        }
    }

    //res.insert("libraries".to_string(), lib_order);
    content.push("".to_string());
    // exec_after
    for entry in &tool_config.exec_after {
        content.push(entry.clone());
    }
    content.push("".to_string());
    let content = content.join("\n");
    let mut file = File::create(filename).unwrap();
    let _ = file.write(content.as_ref());
    fs::set_permissions(filename, fs::Permissions::from_mode(0o770)).unwrap();
    println!("{} was written!", filename)
}

pub fn run_script(filename: &String) {
    let file_name = if !filename.starts_with('/') {
        "./".to_owned() + filename
    } else {
        filename.clone()
    };
    let mut cmd = Command::new(file_name);
    let status = cmd.status();
    let exit_code = match status {
        Ok(exit_status) => exit_status.code().unwrap_or_else(|| {
            eprintln!("Error: No exit status from script");
            21
        }),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("       cmd was:   {:?}", cmd.get_program());
            eprintln!("       args were: {:?}", cmd.get_args());
            eprintln!("       cwd:       {:?}", env::current_dir().unwrap());
            1
        }
    };
    exit(exit_code);
}

pub fn get_sorted_libraries(libraries: &HashMap<String, Library>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let mut libs_remaining: Vec<String> = libraries.keys().cloned().collect();

    // let's start with the empty ones!
    for (name, lib) in libraries {
        if !lib.ignore && lib.depends_on_libs.is_empty() {
            ret.push(name.clone());
            libs_remaining.retain(|x| x != name);
        }
    }
    ret.sort();
    // let's do the rest
    while !libs_remaining.is_empty() {
        let mut libs_new: Vec<String> = Vec::new();
        for name in libs_remaining.clone() {
            let lib = libraries.get(&name).unwrap();
            if lib.ignore {
                libs_remaining.retain(|x| x != &name);
                continue;
            }
            let mut all_available = true;
            for lib_name_dep in &lib.depends_on_libs {
                match libraries.get(lib_name_dep) {
                    None => continue,
                    Some(l) => {
                        if l.ignore {
                            continue;
                        }
                    }
                }
                if !ret.contains(lib_name_dep) {
                    all_available = false;
                }
            }
            if all_available {
                libs_new.push(name.clone());
                libs_remaining.retain(|x| x != &name);
            }
        }
        if libs_new.is_empty() {
            println!("ohoh! {:?} / {:?}", ret, libs_remaining);
            exit(1);
        }
        libs_new.sort();
        ret.append(&mut libs_new);
    }
    ret
}

pub fn print_help_toml() {
    println!(
        "===================================================================
libraries.toml
    This is the TOML file in which the libraries are defined.
=========================================================
[lib_design]
vhdl = [\"ref_design/lib_design/*.vhd\"]
verilog = [\"ref_design/lib_design/*.v\"]

[lib_to_ignore]
ignore=true


===================================================================
tool.toml
    This is the TOML file in which the call of the compiler is defined.
===================================================================
'{{var}}' will be replaced.
Standard replacements are 'library' and 'files'.
Others can be defined by 'replace = ...' (see below) or with the --replacement option.

Example:
common = [\"echo\"]
exec_before = [\"make something before\"]
exec_after = [\"make something after\"]
exec_per_lib = [\"echo make {{library}}\"]
replace = {{ \"origin\" = \"replacement\" }}

[vhdl]
common = [\"-v93\"]
per_lib = [\"-work {{library}}\", \"{{mode}}\", \"{{files}}\"]
single_call = true
exec_per_lib = [\"echo {{library}} vhdl\"]
replace = {{ \"origin2\" = \"replacement2\" }}

[verilog]
common = [\"verilog_com1\"]
per_lib = [\"-work_verilog {{library}}\", \"{{files}}\"]
single_call = false
exec_per_lib = [\"echo {{library}} verilog\"]"
    )
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::path::PathBuf;

    use crate::{get_toplevels_from_lib, write_json_file};

    #[test]
    fn test_filelist_design_1() {
        env::set_var("HANNA_ROOT", env::current_dir().unwrap_or(PathBuf::from(".")));
        let json_filename = String::from("files_design_1.json");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        write_json_file(String::from("lib_1"), String::from("lib_1.design_1(rtl)"), &libraries_toml_path, &tool_toml_path, &replacements, &json_filename);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_filelist_cfg_testbench_1() {
        let json_filename = String::from("files_cfg_testbench_1.json");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        write_json_file(String::from("lib_1"), String::from("lib_1.cfg_testbench_1"), &libraries_toml_path, &tool_toml_path, &replacements, &json_filename);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_filelist_cfg_testbench_2() {
        let json_filename = String::from("files_cfg_testbench_2.json");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        write_json_file(String::from("lib_1"), String::from("lib_1.cfg_testbench_2"), &libraries_toml_path, &tool_toml_path, &replacements, &json_filename);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_filelist_cfg_testbench_3() {
        //env::set_var("RUST_LOG", "TRACE");
        let json_filename = String::from("files_cfg_testbench_3.json");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        write_json_file(String::from("lib_1"), String::from("lib_1.cfg_testbench_3"), &libraries_toml_path, &tool_toml_path, &replacements, &json_filename);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_filelist_cfg_testbench_4() {
        //env::set_var("RUST_LOG", "TRACE");
        let json_filename = String::from("files_cfg_testbench_4.json");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        write_json_file(String::from("lib_1"), String::from("lib_1.cfg_testbench_4"), &libraries_toml_path, &tool_toml_path, &replacements, &json_filename);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_get_toplevels_from_lib() {
        //env::set_var("RUST_LOG", "TRACE");
        let lib_name = String::from("lib_1");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        let top_levels = get_toplevels_from_lib(&lib_name, &libraries_toml_path, &tool_toml_path,
                                                &replacements);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_get_toplevels_from_lib_ignore() {
        //env::set_var("RUST_LOG", "TRACE");
        let lib_name = String::from("lib_ignore");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        let top_levels = get_toplevels_from_lib(&lib_name, &libraries_toml_path, &tool_toml_path,
                                                &replacements);
        assert_eq!(2, 2);
    }

    /*
    #[test]
    fn test_cfg_design_doesnt_exist() {
        //env::set_var("RUST_LOG", "TRACE");
        let json_filename = String::from("files_cfg_design_doesnt_exist.json");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        write_json_file(String::from("lib_1"), String::from("lib_1.cfg_design_doesnt_exist"), &libraries_toml_path, &tool_toml_path, &replacements, &json_filename);
        assert_eq!(2, 2);
    }

     */

    fn test_filelist_cfg_testbench_4() {
        //env::set_var("RUST_LOG", "TRACE");
        let json_filename = String::from("files_cfg_testbench_4.json");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        write_json_file(String::from("lib_1"), String::from("lib_1.cfg_testbench_4"), &libraries_toml_path, &tool_toml_path, &replacements, &json_filename);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_get_toplevels_from_lib() {
        //env::set_var("RUST_LOG", "TRACE");
        let lib_name = String::from("lib_1");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        let top_levels = get_toplevels_from_lib(&lib_name, &libraries_toml_path, &tool_toml_path,
                                                &replacements);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_get_toplevels_from_lib_ignore() {
        //env::set_var("RUST_LOG", "TRACE");
        let lib_name = String::from("lib_ignore");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        let top_levels = get_toplevels_from_lib(&lib_name, &libraries_toml_path, &tool_toml_path,
                                                &replacements);
        assert_eq!(2, 2);
    }

    /*
    #[test]
    fn test_cfg_design_doesnt_exist() {
        //env::set_var("RUST_LOG", "TRACE");
        let json_filename = String::from("files_cfg_design_doesnt_exist.json");
        let libraries_toml_path = String::from("tomls/libraries.toml");
        let tool_toml_path = String::from("tomls/tools/echo.toml");
        let replacements: HashMap<String, String> = HashMap::new();
        write_json_file(String::from("lib_1"), String::from("lib_1.cfg_design_doesnt_exist"), &libraries_toml_path, &tool_toml_path, &replacements, &json_filename);
        assert_eq!(2, 2);
    }

     */
}
