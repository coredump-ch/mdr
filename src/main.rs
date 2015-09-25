extern crate hoedown;
extern crate ansi_term;

use std::io::Write;
use hoedown::{Markdown, Html, Buffer};
use hoedown::renderer::{html, Render};
use ansi_term::Colour::{White, Yellow};
use ansi_term::Style;


struct AnsiTerm {
    name: String,
}

impl Render for AnsiTerm {

    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
        let text = content.to_str().unwrap();
        let header = format!("# {}\n\n", text);
        let formatted = Yellow.bold().paint(&header);
        output.write(&formatted.to_string().into_bytes());
    }

    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
        output.pipe(content);
        output.write(&[0x0a]);
    }

    fn emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let text = content.to_str().unwrap();
        let formatted = White.italic().paint(&text);
        output.write(&formatted.to_string().into_bytes());
        true // Why?
    }

    fn double_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let text = content.to_str().unwrap();
        let formatted = White.bold().paint(&text);
        output.write(&formatted.to_string().into_bytes());
        true // Why?
    }

}

fn main() {
    let doc = Markdown::new("# hey there!\nSome _emphasis_ **is** required.");
    let mut terminal = AnsiTerm { name: "uxterm".to_string() };
    
    println!("{}", terminal.render(&doc).to_str().unwrap());
}
