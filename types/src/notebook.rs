use super::page::Page;


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Notebook {
    chapters: Vec<Chapter>,
    title: String,
}


impl Notebook {
    pub fn from_path(path: impl AsRef<str>) -> Self {
        todo!();
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pages: Vec<Page>,
    title: String,
}


pub struct SelectionOption {
    pub identifier: String,
    pub path: String,
    pub locked: bool,
}


impl SelectionOption {
    pub fn new(identifier: String, path: String, locked: bool) -> Self {
        return Self { identifier, path, locked };
    }
}


pub fn read_selection_options(directory: impl AsRef<str>) -> Vec<SelectionOption> {
    let mut result = Vec::new();

    // TODO: Actually Read From Directory
    result.push( SelectionOption::new("Notebook #1".to_string(), String::new(), false) );
    result.push( SelectionOption::new("Notebook #2".to_string(), String::new(), false) );
    result.push( SelectionOption::new("Notebook #3".to_string(), String::new(), true) );

    return result;
}
