
use comrak::{markdown_to_html, Options};

pub fn to_html(markdown: &str) -> String {
    markdown_to_html(markdown, &Options::default())
}
