use comrak::{markdown_to_html, ComrakOptions};

fn main() {
    let md_content = r#"
## Rust Introduction

Rust is a programming language.

### Hello World

'''rust
fn main() {
    println!("Hello, world!");
}
'''

| First Header  | Second Header |
| ------------- | ------------- |
| Content Cell  | Content Cell  |
| Content Cell  | Content Cell  |

### Reference
Rust official website: www.rust-lang.org

    "#;

    let option = ComrakOptions::default();
    let rendered = markdown_to_html(md_content, &option);
    println!("{}", rendered);

    println!("{}", "-".repeat(20));

    // add ext_autolink option, link will be auto wrapped by a tag
    let option = ComrakOptions {
        ext_autolink: true,
        ..ComrakOptions::default()
    };
    let rendered = markdown_to_html(md_content, &option);
    println!("{}", rendered);
}
