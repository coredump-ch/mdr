extern crate hoedown;
extern crate ansi_term;

use std::io::{Read, Write};
use std::fs::File;
use std::iter::repeat;
use std::{env, process};
use hoedown::{Markdown, Html, Buffer};
use hoedown::renderer::{html, list, Render};
use ansi_term::{Colour, Style};


struct AnsiTerm {
    name: String,
}

impl Render for AnsiTerm {

    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
        // Get prefix and text
        let prefix = repeat("#").take(level as usize).collect::<String>();
        let text = content.to_str().unwrap();

        // Combine prefix and text
        let header = format!("{} {}\n\n", prefix, text);

        // Format and write to terminal
        let formatted = Colour::Yellow.bold().paint(&header);
        output.write(&formatted.to_string().into_bytes());
    }

    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
        output.pipe(content);
        output.write(&[0x0a, 0x0a]);
    }

    fn emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let text = content.to_str().unwrap();
        let formatted = Colour::White.italic().paint(&text);
        output.write(&formatted.to_string().into_bytes());
        true // Why?
    }

    fn double_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let text = content.to_str().unwrap();
        let formatted = Colour::White.bold().paint(&text);
        output.write(&formatted.to_string().into_bytes());
        true // Why?
    }

    fn triple_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let text = content.to_str().unwrap();
        let formatted = Colour::White.bold().underline().paint(&text);
        output.write(&formatted.to_string().into_bytes());
        true // Why?
    }

    fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {
        output.pipe(text);
        output.write(&[0x0a]);
    }

    fn link(&mut self, output: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
        // Read text and target from buffers
        let text = content.to_str().unwrap();
        let target = link.to_str().unwrap();

        // Format text and target
        let formatted_text = Colour::Cyan.underline().paint(&text);
        let formatted_target = Colour::Red.paint(&target);

        output.write(&format!("[{}]({})", formatted_text, formatted_target).to_string().into_bytes());
        true // Why?
    }

    fn code_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
        let text = text.to_str().unwrap();
        let formatted = Colour::Black.on(Colour::White).paint(&text);
        output.write(&formatted.to_string().into_bytes());
        true // Why?
    }

    fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: list::List) {
        output.pipe(&content);
    }

    fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: list::List) {
        output.write(&"- ".as_bytes());
        output.pipe(&content);
    }


}

/// Read file contents, return them as a string.
fn get_file_contents(filepath: &str) -> Result<String, String> {
    let mut file = try!(File::open(filepath).map_err(|msg| format!("Could not open file: {}", msg)));

    let mut s = String::new();
    try!(file.read_to_string(&mut s).map_err(|msg| format!("Could not read file: {}", msg)));

    Ok(s)
}

fn main() {
    // Parse arguments
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} /path/to/markdownfile.md", args[0]);
        process::exit(1);
    }

    // Read file contents
    let text = get_file_contents(&args[1]).unwrap_or_else(|msg| {
        println!("{}", msg);
        process::exit(1);
    });

    // Parse markdown
    let md = Markdown::new(&text);

    // Create AnsiTerm instance
    let mut terminal = AnsiTerm { name: "uxterm".to_string() };
    
    // Print formatted contents to terminal
    println!("{}", terminal.render(&md).to_str().unwrap());
}
