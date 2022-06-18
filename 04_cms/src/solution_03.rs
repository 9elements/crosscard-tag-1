#![allow(unused)]

// SOLUTION STEP 3

struct Block {
    title: Option<Title>,
    content: Content,
}

#[derive(Clone)]
struct Title {
    content: String,
    level: u32,
}

enum Content {
    Text(String),
    Image { url: String, alt: String },
}

struct Page {
    title: String,
    content: Vec<Block>,
}

trait ToHtml {
    fn to_html(&self) -> String;
}

impl ToHtml for Title {
    fn to_html(&self) -> String {
        format!(
            "<h{level:?}>{content}</h{level:?}>",
            level = self.level,
            content = self.content
        )
    }
}

impl ToHtml for Content {
    fn to_html(&self) -> String {
        match self {
            Content::Text(content) => format!("<p>{content}</p>"),
            Content::Image { url, alt } => format!("<img href=\"{url}\" alt=\"{alt}\" />"),
        }
    }
}

impl ToHtml for Block {
    fn to_html(&self) -> String {
        let title_str = self
            .title
            .clone()
            .map(|title| title.to_html())
            .unwrap_or_default();

        let content = self.content.to_html();

        format!("{title_str}{content}")
    }
}

impl ToHtml for Page {
    fn to_html(&self) -> String {
        let body: String = self.content.iter().map(ToHtml::to_html).collect();

        format!(
            "<!DOCTYPE html>
<html>
<head>
  <title>{title}</title>
</head>
<body>
    {body}
</body>
</html>",
            title = self.title
        )
    }
}

fn print_html<T: ToHtml>(page: T) {
    let rendered = page.to_html();
    println!("{rendered}");
}
