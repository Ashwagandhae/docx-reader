extern crate minidom;
extern crate serde;
extern crate zip;

use minidom::Element;
use minidom::NSChoice;

use zip::read::ZipArchive;

use std::collections::HashMap;
use std::fs;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Style {
    bold: Option<bool>,
    underline: Option<bool>,
    highlight: Option<bool>,
    size: Option<u32>,
}
#[derive(Serialize, Deserialize)]
pub struct Run {
    text: String,
    style: Style,
}
#[derive(Serialize, Deserialize)]
pub struct Para {
    runs: Vec<Run>,
}
pub struct Document {
    pub paras: Vec<Para>,
    pub style_map: HashMap<String, Style>,
}

pub trait Drop {
    fn new() -> Self;
    fn load_file(&mut self, file_path: &str);
    fn load_style_map(&mut self, contents: &mut String);
    fn load_paras(&mut self, contents: &mut String);

    // fn mutate_style(style: &Style, priority_style: &Style) -> Style;
    fn mutate_style_mut(style: &mut Style, priority_style: &Style);

    // fn get_style_id(&mut self, current_style: &Style, element: &Element) -> Style;
    fn get_style_id_mut(&mut self, style: &mut Style, element: &Element);

    // fn get_style(&mut self, current_style: &Style, element: &Element) -> Style;
    fn get_style_mut(&mut self, style: &mut Style, element: &Element);

    fn process_para(&mut self, para: &Element) -> Option<Para>;
    fn process_run(&mut self, run: &Element, current_style: &Style) -> Option<Run>;
}

impl Drop for Document {
    fn new() -> Document {
        Document {
            paras: Vec::new(),
            style_map: HashMap::new(),
        }
    }
    fn load_file(&mut self, file_path: &str) {
        let file = fs::File::open(file_path).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        println!("unzipped file");
        let mut style = archive.by_name("word/styles.xml").unwrap();
        println!("got style file");
        let buf = &mut String::new();
        match style.read_to_string(buf) {
            Ok(_) => self.load_style_map(buf),
            Err(e) => println!("Error reading styles.xml: {}", e),
        }
        drop(style);
        let mut document = archive.by_name("word/document.xml").unwrap();
        println!("got document file");
        let buf = &mut String::new();
        match document.read_to_string(buf) {
            Ok(_) => self.load_paras(buf),
            Err(e) => println!("Error reading document.xml: {}", e),
        }
    }
    fn load_style_map(&mut self, contents: &mut String) {
        let root: Element = contents.parse().unwrap();
        let ns = NSChoice::Any;
        for child in root.children() {
            if child.is("style", ns) {
                let id = child.attr("w:styleId").unwrap().to_string();
                let mut style = Style {
                    bold: None,
                    underline: None,
                    highlight: None,
                    size: None,
                };
                self.get_style_mut(&mut style, child);
                self.style_map.insert(id, style);
            }
        }
        println!("loaded style map")
    }
    fn load_paras(&mut self, contents: &mut String) {
        let root: Element = contents.parse().unwrap();
        println!("parsed paras");
        let ns = NSChoice::Any;

        let body = root.get_child("body", ns);
        if body.is_some() {
            for child in body.unwrap().children() {
                let para = self.process_para(child);
                if (para).is_some() {
                    self.paras.push(para.unwrap());
                }
            }
        }
        println!("loaded paragraphs")
    }
    // fn mutate_style(style: &Style, priority_style: &Style) -> Style {
    //     let mut new_style = style.clone();
    //     new_style.bold = priority_style.bold.or(style.bold);
    //     new_style.underline = priority_style.underline.or(style.underline);
    //     new_style.highlight = priority_style.highlight.or(style.highlight);
    //     new_style.size = priority_style.size.or(style.size);
    //     return new_style;
    // }
    fn mutate_style_mut(style: &mut Style, priority_style: &Style) {
        style.bold = priority_style.bold.or(style.bold);
        style.underline = priority_style.underline.or(style.underline);
        style.highlight = priority_style.highlight.or(style.highlight);
        style.size = priority_style.size.or(style.size);
    }
    // fn get_style_id(&mut self, current_style: &Style, element: &Element) -> Style {
    //     let mut style = current_style.clone();
    //     let id_style = self.style_map.get(element.attr("w:val").unwrap());
    //     if id_style.is_some() {
    //         Self::mutate_style_mut(&mut style, &id_style.unwrap());
    //     }
    //     return style;
    // }
    fn get_style_id_mut(&mut self, style: &mut Style, element: &Element) {
        let id_style = self.style_map.get_mut(element.attr("w:val").unwrap());
        if id_style.is_some() {
            Self::mutate_style_mut(style, &id_style.unwrap());
        }
    }
    // fn get_style(&mut self, current_style: &Style, element: &Element) -> Style {
    //     // copy current_style into style
    //     let mut style = current_style.clone();
    //     for child in element.children() {
    //         match child.name() {
    //             "b" => style.bold = Some(child.attr("w:val").unwrap_or("1") != "0"),
    //             "u" => style.underline = Some(child.attr("w:val").unwrap_or("single") != "none"),
    //             "highlight" => style.highlight = Some(true),
    //             "sz" => style.size = Some(child.attr("w:val").unwrap().parse::<u32>().unwrap()),
    //             "rPr" => self.get_style_mut(&mut style, child),
    //             "pPr" => self.get_style_mut(&mut style, child),
    //             "rStyle" => self.get_style_id_mut(&mut style, child),
    //             "pStyle" => self.get_style_id_mut(&mut style, child),
    //             "basedOn" => self.get_style_id_mut(&mut style, child),
    //             _ => (),
    //         };
    //     }
    //     return style;
    // }
    fn get_style_mut(&mut self, style: &mut Style, element: &Element) {
        for child in element.children() {
            match child.name() {
                "b" => style.bold = Some(child.attr("w:val").unwrap_or("1") != "0"),
                "u" => style.underline = Some(child.attr("w:val").unwrap_or("single") != "none"),
                "highlight" => style.highlight = Some(true),
                "sz" => style.size = Some(child.attr("w:val").unwrap().parse::<u32>().unwrap()),
                "rPr" => self.get_style_mut(style, child),
                "pPr" => self.get_style_mut(style, child),
                "rStyle" => self.get_style_id_mut(style, child),
                "pStyle" => self.get_style_id_mut(style, child),
                "basedOn" => self.get_style_id_mut(style, child),
                _ => (),
            };
        }
    }
    fn process_para(&mut self, para: &Element) -> Option<Para> {
        let ns = NSChoice::Any;
        if !para.is("p", ns) {
            return None;
        }
        let mut runs: Vec<Run> = Vec::new();
        let mut current_style = Style {
            bold: Some(false),
            underline: Some(false),
            highlight: Some(false),
            size: Some(0),
        };
        for child in para.children() {
            if child.is("r", ns) {
                let run = self.process_run(child, &current_style);
                if run.is_some() {
                    runs.push(run.unwrap());
                }
            } else if child.is("pPr", ns) {
                self.get_style_mut(&mut current_style, child);
            }
        }
        return Some(Para { runs: runs });
    }
    fn process_run(&mut self, run: &Element, current_style: &Style) -> Option<Run> {
        let mut style = current_style.clone();
        let ns = NSChoice::Any;
        if !run.is("r", ns) {
            return None;
        }
        let mut texts: Vec<String> = Vec::new();
        for child in run.children() {
            if child.is("t", ns) {
                let text = child.text();
                if text.len() > 0 {
                    texts.push(text);
                }
            } else if child.is("rPr", ns) {
                self.get_style_mut(&mut style, child);
            }
        }
        if !texts.is_empty() {
            return Some(Run {
                text: texts.join(""),
                style: style.clone(),
            });
        }
        return None;
    }
}

// let mut document = Document::new();
// document.load_file("./test.docx");
// for para in document.paras {
//     for run in para.runs {
//         println!("s{}s", run.text);
//         // print style with labels
//         if run.style.bold.unwrap_or(false) {
//             println!("Bold");
//         }
//         if run.style.underline.unwrap_or(false) {
//             println!("Underline");
//         }
//         if run.style.highlight.unwrap_or(false) {
//             println!("highlight");
//         }
//         println!("Size: {}", run.style.size.unwrap());
//         println!("\n");
//     }
// }
