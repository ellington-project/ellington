extern crate xml; 
extern crate plist; 

use plist::Plist; 

use std::fs::File; 
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent}; 
use xml::reader::XmlEvent::CData;
use xml::reader::XmlEvent::Comment;
use xml::reader::XmlEvent::ProcessingInstruction;
use xml::reader::XmlEvent::Whitespace;


struct Track {
    bpm: u32,
    name: String, 
    location: String,  
}



fn print_library(filename: &str) -> () {
    let file = File::open(filename).unwrap(); 
    let file = BufReader::new(file); 

    let parser = EventReader::new(file);
    // let mut books = Vec::new();
    let mut _current_element = String::new();

    let mut indent = "".to_string();
    for e in parser {
        match e {
            Ok(XmlEvent::StartDocument { ..}) => {
                println!("{}StartDocument", indent);
                indent.push('\t');
            },
            Ok(XmlEvent::EndDocument {.. }) => {
                indent.pop();
                println!("{}EndDocument", indent);
            },
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}StartElement: {}", indent, name);
                indent.push('\t');
            },
            Ok(XmlEvent::EndElement { name,  .. }) => {
                indent.pop();
                println!("{}EndElement: {}", indent, name);
            },
            Ok(CData(_)) => {

            },
            Ok(Comment(_)) => {

            }, 
            Ok(ProcessingInstruction { .. }) => {

            },
            Ok(Whitespace(_)) => {

            },
            Ok(XmlEvent::Characters(s)) => {
                println!("{}Characters: {}", indent, s);
            },
            Err(e) => {
                println!("{}Error: {}", indent, e)
            }
            // _ => {
            //     println!("Failed to parse: {}", e);
            // }
        }
    }

}

fn read_plist(filename: &str) -> () {
    let file = File::open(filename).unwrap();

    let plist = Plist::read(file).unwrap(); 

    match plist {
        Plist::Array(_array) => println!("Array"), 
        Plist::Dictionary(_) => println!("Dictionary"), 
        Plist::Boolean(_) => println!("Boolean"), 
        Plist::Data(_) => println!("Data"),
        Plist::Date(_) => println!("Date"),
        Plist::Real(_) => println!("Real"), 
        Plist::Integer(_) => println!("Integer"), 
        Plist::String(_) => println!("String")
        // _ => ()
    }
}

fn main() {
    // print_library("res/partialLibrary.xml");
    read_plist("res/partialLibrary.xml");
    println!("Hello, world!");
}
