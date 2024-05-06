use std::collections::HashMap;
use std::env;
use std::process::exit;
use std::string::String;

use clap::{CommandFactory, Parser};
use env_logger::Target;
use log::{debug, error, trace, warn};

use hanna::{gen_script, get_library_names_from_toml, get_toplevels_from_lib, print_help_toml, run_script, write_json_file, write_lib_lists};
use hanna::classes::RE_ENT;

use crate::cli::ArgsStruct;
use crate::cli::Cli;
use crate::cli::Commands;

pub mod cli;

fn main() {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .init();

    //
    trace!("called in {:?}",env::current_dir());
    //
    let cli_args = Cli::parse();

    let args: ArgsStruct;
    match cli_args.command {
        None => {
            if cli_args.about {
                //println!("Hanna is written by {}.\nSee https://github.com/HeikoBraun/hanna", clap::crate_authors!(", "));
                println!("See https://github.com/HeikoBraun/hanna");
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
                        list_only: fc.list_only,
                        use_work: false,
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
                        list_only: false,
                        use_work: false,
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
                        list_only: false,
                        use_work: false,
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
                        list_only: false,
                        use_work: sc.work,
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
                        list_only: false,
                        use_work: sc.work,
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
    //let tool_toml = read_tool_toml(&args.tool, &replacements);
    //
    //let libraries = read_libraries_toml(&args.libraries, &replacements, &tool_toml);
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

    match args.command.as_str() {
        "info" => {
            if lib_name.is_empty() {
                let libs_list = get_library_names_from_toml(&args.libraries, &replacements);
                if args.list_only {
                    println!("{}", libs_list.join("\n"));
                } else if libs_list.is_empty() {
                    println!("No libraries defined in {}", &args.libraries);
                } else {
                    println!("Libraries defined:\n - {}", libs_list.join("\n - "));
                }
            } else {
                let top_levels = get_toplevels_from_lib(&lib_name, &args.libraries, &args.tool,
                                                        &replacements);
                if top_levels.is_empty() {
                    println!("No top levels found in library {}", lib_name);
                } else {
                    println!(
                        "Library {} contains following top levels:\n - {}",
                        lib_name,
                        top_levels.join("\n - ")
                    )
                }
            }
        }
        _ => {
            match args.command.as_str() {
                "files" => {
                    write_lib_lists(lib_name, args.toplevel, &args.libraries, &args.tool, &replacements, &args.filename);
                }
                "json" => {
                    write_json_file(lib_name, args.toplevel, &args.libraries, &args.tool, &replacements, &args.filename);
                }
                "script" => {
                    gen_script(lib_name, args.toplevel, &args.libraries, &args.tool, &replacements, &args.filename, args.use_work);
                }
                "execute" => {
                    gen_script(lib_name, args.toplevel, &args.libraries, &args.tool, &replacements, &args.filename, args.use_work);
                    run_script(&args.filename);
                }
                _ => {
                    warn!("{} is not implemented yet", args.command)
                }
            }
        }
    }
}
