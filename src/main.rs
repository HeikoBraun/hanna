use std::collections::HashMap;
use std::process::exit;
use std::string::String;

use clap::{CommandFactory, Parser};
use env_logger::Target;
use log::{debug, error, warn};

use crate::classes::*;
use crate::classes::aux_functions::{
    gen_script, get_sorted_libraries, print_help_toml, read_libraries_toml, read_tool_toml,
    run_script, write_json_file, write_lib_lists,
};
use crate::cli::ArgsStruct;
use crate::cli::Cli;
use crate::cli::Commands;

pub mod classes;
pub mod cli;

fn main() {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .init();

    //
    let cli_args = Cli::parse();

    let args: ArgsStruct;
    match cli_args.command {
        None => {
            if cli_args.about {
                println!("Hanna is written by {}.\nSee https://github.com/HeikoBraun/hanna", clap::crate_authors!(", "));
            } else if cli_args.help_toml {
                print_help_toml();
            } else {
                // show help
                let mut cmd = Cli::command();
                cmd.print_help().expect("No help available");
            }
            exit(0)
        }
        Some(subcommand) => {
            match subcommand {
                Commands::Info(fc) => {
                    args = ArgsStruct {
                        command: "info".to_string(),
                        toplevel: fc.library,
                        libraries: fc.libraries,
                        tool: fc.tool,
                        replacement: fc.replacement,
                        filename: String::new(),
                        forces: Vec::new(),
                    }
                }
                Commands::Files(fc) => {
                    args = ArgsStruct {
                        command: "files".to_string(),
                        toplevel: fc.toplevel,
                        libraries: fc.libraries,
                        tool: fc.tool,
                        replacement: fc.replacement,
                        filename: fc.path,
                        forces: fc.force,
                    }
                }
                Commands::Json(jc) => {
                    args = ArgsStruct {
                        command: "json".to_string(),
                        toplevel: jc.toplevel,
                        libraries: jc.libraries,
                        tool: jc.tool,
                        replacement: jc.replacement,
                        filename: jc.name,
                        forces: jc.force,
                    }
                }
                Commands::Script(sc) => {
                    args = ArgsStruct {
                        command: "script".to_string(),
                        toplevel: sc.toplevel,
                        libraries: sc.libraries,
                        tool: sc.tool,
                        replacement: sc.replacement,
                        filename: sc.name,
                        forces: sc.force,
                    }
                }
                Commands::Execute(sc) => {
                    args = ArgsStruct {
                        command: "execute".to_string(),
                        toplevel: sc.toplevel,
                        libraries: sc.libraries,
                        tool: sc.tool,
                        replacement: sc.replacement,
                        filename: sc.name,
                        forces: sc.force,
                    }
                }
            };
        }
    };

    //
    let mut replacements: HashMap<String, String> = HashMap::new();
    for replacement in args.replacement {
        let parts: Vec<&str> = replacement.split('=').collect();
        if parts.len() == 2 {
            replacements.insert(parts[0].to_string(), parts[1].to_string());
        } else {
            warn!(
                "replacement '{}' seems not like a valid 'foo=bar' replacement option",
                replacement
            );
        }
    }

    //
    let tool_toml = read_tool_toml(&args.tool, &replacements);
    //for (key, value) in &tool_toml.replacement {
    //    replacements.insert(key.into(), value.into());
    //}
    //
    let libraries = read_libraries_toml(&args.libraries, &replacements, &tool_toml);
    let lib_name: String;

    if args.command == "info" {
        lib_name = args.toplevel.clone();
    } else {
        match RE_ENT.captures(&args.toplevel) {
            None => {
                error!("No valid search pattern: {}", args.toplevel);
                exit(1)
            }
            Some(caps) => {
                match caps.name("lib") {
                    None => {
                        error!("lib name is mandatory");
                        exit(1)
                    }
                    Some(_) => {}
                };
                lib_name = caps["lib"].to_string();
                let rest = caps["rest"].to_string();
                debug!("Lib: {} Rest: {}", lib_name, rest);
            }
        }
    };
    let empty_lib = Library::new();
    let lib = libraries.get(&*lib_name).unwrap_or_else(|| {
        if lib_name.is_empty() {
            &empty_lib
        } else {
            error!("library '{}' is unknown", lib_name);
            exit(1)
        }
    });

    match args.command.as_str() {
        "info" => {
            if lib_name.is_empty() {
                let mut libs: Vec<String> = Vec::new();
                for (key, lib) in &libraries {
                    if !lib.ignore {
                        libs.push(key.clone());
                    }
                }
                libs.sort();
                println!("Available libraries:\n{}", libs.join("\n"));
                for (key, lib) in &libraries {
                    if !lib.ignore {
                        debug!("Lib: {}: {:?}", key, lib.depends_on_libs);
                    }
                }
                debug!("Libs sorted: {:?}", get_sorted_libraries(&libraries));
            } else {
                println!(
                    "Library {} contains following top levels:\n{}",
                    lib_name,
                    lib.list_designs().join("\n")
                )
            }
        }
        _ => {
            let mut el_list: Vec<Element> = Vec::new();
            if !args.forces.is_empty() {
                for force in &args.forces {
                    let lib = match libraries.get(force) {
                        None => {
                            error!("library '{}' is unknown", force);
                            exit(1)
                        }
                        Some(lib) => lib,
                    };
                    for el in &lib.all_verilog_elements {
                        el_list.push(el.copy());
                    }
                    for el in &lib.all_vhdl_elements {
                        el_list.push(el.copy());
                    }
                }
            }
            el_list.extend(lib.resolve(&args.toplevel, &libraries));
            let lib_order = get_sorted_libraries(&libraries);
            match args.command.as_str() {
                "files" => {
                    write_lib_lists(&el_list, lib_order, &args.filename);
                }
                "json" => {
                    write_json_file(&el_list, lib_order, &args.filename);
                }
                "script" => {
                    gen_script(&el_list, lib_order, &args.filename, &tool_toml);
                }
                "execute" => {
                    gen_script(&el_list, lib_order, &args.filename, &tool_toml);
                    run_script(&args.filename);
                }
                _ => {
                    warn!("{} is not implemented yet", args.command)
                }
            }
        }
    }
}
