use std::fs;

use super::page::Page;


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Notebook {
    chapters: Vec<Chapter>,
    title: String,
}


impl Notebook {
    pub fn from_path(path: impl AsRef<str>) -> Result<Self, &'static str> {
        let file = fs::read_to_string(path.as_ref())
            .map_err(|_| "Failed to read notebook.")?;

        let object = ron::from_str(file.as_str())
            .map_err(|_| "Failed to deserialize into `Notebook`.")?;

        return Ok(object);
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pages: Vec<Page>,
    title: String,
}


#[derive(Debug, Clone)]
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
