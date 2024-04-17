# Hanna

HDL analyzer and compile script generator.

## Purpose

Analyzes VHDL files and generates file lists or
compile scripts for a certain target.

## Usage

Usage: hanna [OPTIONS] [COMMAND]

Commands:

info     
files    
json     
script   
execute  
help

Options:

--help-toml  
-h, --help
--version

See 'hanna help <command you are interested in>' for further details on that command.

Most of them have:

* -l \<path to libraries.toml\>
* -t \<path to tool.toml\>
* \<toplevel\>, which must be of format lib_name.cfg_top or lib_name.top(arch)
  . IMHO using configurations is a good coding style.

## How to configure

Two TOML files are used to configure hanna.

${ENVVAR} will be replaced by the corresponding environment variable.

'{var}' will be replaced also.
Standard replacements are 'library' and 'files'.
Others can be defined by 'replace = ...' (see below)
or with the --replacement option when calling hanna.

### libraries.toml

In this file the libraries and their source code
locations are defined.

Syntax:

```
[name_of_library]
vhdl = ["ref_design/lib_design/**/*.vhd"]
verilog = ["ref_design/lib_design/*.v"] 

[name_of_library_to_ignore]
ignore = true
```

### tool.toml

In this file the usage of the tool
for the script generation is defined.
All are optional.

Syntax:

```
# common section for VHDL and Verilog which are all optional
common = ["echo"]                           # common part/name of compile command
exec_before = ["make something before"]     # commands to run before start of compilation
exec_after = ["make something after"]       # commands to run after compilation
exec_per_lib = ["echo make {library}"]      # execute this command for each library before compile
replace = { "somehing" = "else" }           # replace occurences of {something} in the TOML files with 'else'

# VHDL section
[vhdl]
common = ["-v93"]                                   # common part (extending above) for compile command
per_lib = ["-work {library}", "{mode}", "{files}"]  # part (extending above) for each library
single_call = true                                  # wether all can be done in one compile command call or need one for each library.
exec_per_lib = ["echo {library} vhdl"]              # execute this command for each library before compile
replace = { "somehing_2" = "else_2" }               # replace occurences of {something_2} in the TOML files with 'else_2'

# Verilog section
[verilog]                                           
common = ["verilog_com1"]                           # common part (extending above) for compile command
per_lib = ["-work_verilog {library}", "{files}"]    # part (extending above) for each library
single_call = false                                 # wether all can be done in one compile command call or need one for each li
exec_per_lib = ["echo {library} verilog"]"          # execute this command for each library before compile
```
