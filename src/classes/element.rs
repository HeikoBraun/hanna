pub struct Element {
    pub library: String,
    pub filename: String,
    pub language: String,
}

impl Element {
    pub fn copy(&self) -> Element {
        Element {
            library: self.library.clone(),
            filename: self.filename.clone(),
            language: self.language.clone(),
        }
    }
}

/*
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Element: Library: {} Filename: {} Language: {}",
            self.library, self.filename, self.language
        )
    }
}
*/