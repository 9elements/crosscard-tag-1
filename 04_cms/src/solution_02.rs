#![allow(unused)]

// SOLUTION STEP 2

struct Block {
    title: Option<Title>,
    content: Content,
}

struct Title {
    content: String,
    level: u32,
}

enum Content {
    Text(String),
    Image { url: String, alt: String },
}
