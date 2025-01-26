// src/lib/domain.rs

// domain data types and methods

// dependencies
use serde::{Deserialize, Serialize};

// struct type to represent an individual blog post
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BlogPost {
    id: i64,
    title: String,
    date: String,
    date_updated: Option<String>,
    draft: bool,
    edited: bool,
    slug: String,
    category: String,
    tag: String,
    summary: String,
    content: String,
}

// methods for the BlogPost struct type
impl BlogPost {
    pub fn new() -> Self {
        Self::default()
    }
}
