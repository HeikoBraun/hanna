use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author)]
#[command(version)]
#[command(about = "VHDL analyzer and compile script generator.")]
#[command(long_about = "VHDL analyzer and compile script generator.\n\
\n\
Analyzes VHDL files of the libraries defined in libraries.toml.\n\
It can create a compile script or some file lists files.")]
pub struct Cli {
    #[arg(long, global = true, hide = true, alias = "author")]
    pub about: bool,

    #[arg(long, global = true)]
    pub help_toml: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // info which designs are in library
    Info(InfoCommands),

    // file lists
    Files(FilesCommands),

    // json file
    Json(JSONCommands),

    // script
    Script(ScriptCommands),

    // execute: not yet, generate script and execute
    Execute(ScriptCommands),
    // tree: not yet
    //Tree(FilesCommands),
}

#[derive(Debug, Args)]
pub struct InfoCommands {
    #[arg(default_value_t = String::from(""))]
    pub library: String,

    /// replacements in toml files of form {var}, format is var=new_value
    #[arg(short, long)]
    pub replacement: Vec<String>,

    /// path to libraries.toml
    #[arg(short, long, default_value_t = String::from("libraries.toml"))]
    pub libraries: String,

    /// path to tool.toml
    #[arg(short, long, default_value_t = String::from("tool.toml"))]
    pub tool: String,

    /// only list, no prosa
    #[arg(long, default_value_t = false)]
    pub list_only: bool,

}

#[derive(Debug, Args)]
pub struct FilesCommands {
    #[arg()]
    pub toplevel: String,

    /// replacements in toml files of form {var}, format is var=new_value
    #[arg(short, long)]
    pub replacement: Vec<String>,

    /// path to libraries.toml
    #[arg(short, long, default_value_t = String::from("libraries.toml"))]
    pub libraries: String,

    /// path to tool.toml
    #[arg(short, long, default_value_t = String::from("tool.toml"))]
    pub tool: String,

    /// path where to output files
    #[arg(short, long, default_value_t = String::from("./"))]
    pub path: String,

    /// force to compile library in arbitrary order (could be useful if Verilog has needed submodules)
    #[arg(short, long)]
    pub force: Vec<String>,

    /// ignore library
    #[arg(long)]
    pub ignore_library: Vec<String>,
}

#[derive(Debug, Args)]
pub struct JSONCommands {
    #[arg()]
    pub toplevel: String,

    /// replacements in toml files of form {var}, format is var=new_value
    #[arg(short, long)]
    pub replacement: Vec<String>,

    /// path to libraries.toml
    #[arg(short, long, default_value_t = String::from("libraries.toml"))]
    pub libraries: String,

    /// path to tool.toml
    #[arg(short, long, default_value_t = String::from("tool.toml"))]
    pub tool: String,

    /// json path
    #[arg(short, long, default_value_t = String::from("libraries.json"))]
    pub name: String,

    /// force to compile library in arbitrary order (could be useful if Verilog has needed submodules)
    #[arg(short, long)]
    pub force: Vec<String>,

    /// ignore library
    #[arg(long)]
    pub ignore_library: Vec<String>,
}

#[derive(Debug, Args)]
pub struct ScriptCommands {
    #[arg()]
    pub toplevel: String,

    /// replacements in toml files of form {var}, format is var=new_value
    #[arg(short, long)]
    pub replacement: Vec<String>,

    /// path to libraries.toml
    #[arg(short, long, default_value_t = String::from("libraries.toml"))]
    pub libraries: String,

    /// path to tool.toml
    #[arg(short, long, default_value_t = String::from("tool.toml"))]
    pub tool: String,

    /// script path
    #[arg(short, long, default_value_t = String::from("compile.sh"))]
    pub name: String,

    /// force to compile library in arbitrary order (could be useful if Verilog has needed submodules)
    #[arg(short, long)]
    pub force: Vec<String>,

    /// compile toplevel to work instead original lib name
    #[arg(long, default_value_t = false)]
    pub work: bool,

    /// ignore library
    #[arg(long)]
    pub ignore_library: Vec<String>,

    /// activate options
    #[arg(long)]
    pub option: Vec<String>,
}

pub struct ArgsStruct {
    pub command: String,
    pub toplevel: String,
    pub libraries: String,
    pub tool: String,
    pub replacements: Vec<String>,
    pub filename: String,
    pub forces: Vec<String>,
    pub list_only: bool,
    pub use_work: bool,
    pub ignore_libraries: Vec<String>,
    pub options: Vec<String>,
}
