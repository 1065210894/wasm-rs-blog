use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct HtmlByTree {
    pub html_type: String,
    pub content: String,
    pub href: String,
    pub child: Vec<HtmlByTree>,
}