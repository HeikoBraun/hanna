use once_cell::sync::Lazy;
use regex::Regex;

pub static RE_COMMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)(?P<comment>--.*?)$").unwrap()
});

pub static RE_STD_LIBS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)^\s*library\s+(ieee|std)\b").unwrap()
});
pub static RE_LIBS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)^\s*library\s+(?P<name>\w+)\s*;").unwrap()
});
pub static RE_USE_STD_LIBS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)^\s*use\s+(ieee|std)\.\S+").unwrap()
});
pub static RE_USE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imx)^\s*use\s+(?P<content>(?P<lib>\w+\.)\S+?)\s*;").unwrap()
});
pub static RE_ENTITY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)
            (?P<start>\bentity\s+(?P<name>\w+)\s+IS\s+)
                (?P<content>.*?)
            (?P<end>\bend(\s*(;|(\s+[^;]*?;))))",
    ).unwrap()
});
pub static RE_ARCHITECTURE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)
            (?P<start>\barchitecture\s+(?P<name>\w+)\s+of\s+(?P<entity>\w+)\s+is\s+)
                (?P<definitions>.*?)
                (?P<content>\bbegin\b.*?)
            (?P<end>\bend(;|\s+[^;]*?;))",
    ).unwrap()
});
pub static RE_CONFIGURATION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)
            \bconfiguration\s+(?P<name>\w+)\s+of\s+(?P<entity>\w+).*?
                \bfor\s+(?P<arch>\w+)
                    (?P<content>.*)
                \bend\s+for\s*;
            \s*\bend\s*(configuration\s*)?(\w+\s*)?;",
    ).unwrap()
});
pub static RE_PROCESS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)
            (?:\w+\s*:\s*)?         # optional process name
            \bprocess\b.*?          # start of process
            \bend\s+process.*?;     # end   of process",
    ).unwrap()
});
pub static RE_SIGNAL_OR_VARIABLE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)\b(?:constant|signal|variable)\b.*?;").unwrap()
});
pub static RE_FUNC_PROC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)\b(?P<type>function|procedure)\b.*?\bbegin\b
            (
                .*?
                |
                (.*?\bend\s+(if|case|loop)\b)
            )+
            \bend\b.*?;",
    ).unwrap()
});
pub static RE_GENERATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imx)^.*\bgenerate\b.*?$").unwrap()
});
pub static RE_INSTANCE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)\b(?P<label>\w+)\s*:\s*
    (
         ((component\s+)?(?P<c_name>\w+))
        |(entity\s+((?P<e_lib>\w+)\.)?(?P<e_name>\S+(\s*\(\s*\S+?\s*\))?))
        |(configuration\s+((?P<c_lib>\w+)\.)?(?P<con_name>\w+))
    )
    ((\s*;)|(\s+(generic|port)))",
    ).unwrap()
});
pub static RE_PACKAGE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)
        \bpackage\s+(?P<name>\w+)\s+is\s+
            (?P<content>.*?)
        \bend.*?;",
    ).unwrap()
});
pub static RE_PACKAGE_BODY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)
        \bpackage\s+body\s+(?P<name>\w+)\s+is\s+
            (?P<content>.*?)
        \bend.*?;",
    ).unwrap()
});
pub static RE_CONF_COMP_SPEC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)
        \bfor\s+(?P<label>\w+(\s*,\s*\w+)*)\s*:\s*(?P<comp>\w+)\s+
            use\s+((?P<open>open)|(configuration\s+(?P<conf>\S+?))|(entity\s+(?P<entity>\S+?)))\s*;\s*
        \bend\s+for\s*;",
    ).unwrap()
});
pub static RE_ENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)((?P<lib>\w+)\.)?(?P<rest>(?P<entity>\w+)(\s*\(\s*(?P<arch>\w+)\s*\))?)").unwrap()
});
pub static RE_ENT2: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<lib>\w+)\.(?P<rest>.*)").unwrap()
});
/*
pub static RE_DESIGN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)((?P<lib>\w+)\.)?((?P<configuration>\w+)|((?P<entity>\w+)\((?P<arch>\w+)\)))",
    ).unwrap()
});
*/

pub static RE_USAGE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)(?P<lib>\w+)\.(?P<package>\w+)(\.(?P<element>\w+))?").unwrap()
});

pub static RE_MODULE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)\bmodule\s+(?P<name>\w+)").unwrap()
});
/*
pub static RE_MODULE_INST: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)\b(?P<name>\w+)\s+\w+\s+\(").unwrap()
});
*/

pub static RE_ENVVAR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\$((?P<var1>\w+)|(\{(?P<var2>[^}]+?)}))").unwrap()
});
