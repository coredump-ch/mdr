extern crate hoedown;
extern crate ansi_term;

use std::io::Write;
use hoedown::{Markdown, Html, Buffer};
use hoedown::renderer::{html, Render};
use ansi_term::Colour::Yellow;
use ansi_term::Style;


struct AnsiTerm {
    name: String,
}

impl Render for AnsiTerm {

    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
        let text = content.to_str().unwrap();
        let header = format!("# {}", text);
        let formatted = Yellow.bold().paint(&header);
        output.write(&formatted.to_string().into_bytes());
    }

}

fn main() {
    let doc = Markdown::new("# hey there!\nsome _emphasis_ *is* required.");
    let mut terminal = AnsiTerm { name: "uxterm".to_string() };
    
    println!("{}", terminal.render(&doc).to_str().unwrap());
}
