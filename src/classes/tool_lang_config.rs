pub struct ToolLangConfig {
    pub common: Vec<String>,
    pub per_lib: Vec<String>,
    pub single_call: bool,
    pub exec_per_lib: Vec<String>,
}
/*
impl Default for ToolLangConfig {
    fn default() -> Self {
        Self::new()
    }
}
*/
impl ToolLangConfig {
    pub fn new() -> ToolLangConfig {
        ToolLangConfig {
            common: Vec::new(),
            per_lib: Vec::new(),
            single_call: false,
            exec_per_lib: Vec::new(),
        }
    }
}
