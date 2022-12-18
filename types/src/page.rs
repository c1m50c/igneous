#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Page {
    contents: Vec<PageComponent>,
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum PageComponent {
    Heading { text: String, size: HeadingSize },
    Paragraph { text: String },
    CodeBlock { text: String },
    UnorderedList { elements: Box<PageComponent> },
    OrderedList { elements: Box<PageComponent> },
    CheckList { elements: Box<PageComponent> },
    Link { address: String },
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum HeadingSize {
    Small,
    Medium,
    Large,
    Title,
}