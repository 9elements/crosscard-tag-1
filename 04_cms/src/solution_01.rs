#![allow(unused)]

// SOLUTION STEP 1

struct Block {
    title: Option<String>,
    content: Content,
}

enum Content {
    Text(String),
    ImageUrl(String),
}
