use std::collections::HashMap;

use crate::classes::ToolLangConfig;

pub struct ToolConfig {
    pub common: Vec<String>,
    pub lang_configs: HashMap<String, ToolLangConfig>,
    pub exec_before: Vec<String>,
    pub exec_after: Vec<String>,
    pub exec_per_lib: Vec<String>,
    pub replacement: HashMap<String, String>,
}
/*
impl Default for ToolConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolConfig {
    pub fn new() -> ToolConfig {
        ToolConfig {
            common: Vec::new(),
            lang_configs : HashMap::new(),
            exec_before: Vec::new(),
            exec_after: Vec::new(),
            exec_per_lib: Vec::new(),
            replacement: HashMap::new(),
        }
    }
}
*/