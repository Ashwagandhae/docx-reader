extern crate serde;
extern crate zip;

use zip::read::ZipArchive;

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::str;

use serde::{Deserialize, Serialize};

use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Style {
    pub bold: Option<bool>,
    pub underline: Option<bool>,
    pub highlight: Option<bool>,
    pub size: Option<u32>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Attr {
    pub style: Style,
    pub outline_level: Option<u32>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Run {
    pub text: String,
    pub style: Style,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Para {
    pub index: usize,
    pub outline_level: Option<u32>,
    pub runs: Vec<Run>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct OutlinePara {
    pub index: usize,
    pub link: usize,
    pub outline_level: Option<u32>,
    pub runs: Vec<Run>,
}
pub struct Document {
    pub paras: Vec<Para>,
    pub outline_paras: Vec<OutlinePara>,
    pub style_map: HashMap<String, Attr>,
}
impl Document {
    pub fn new() -> Document {
        Document {
            outline_paras: Vec::new(),
            paras: Vec::new(),
            style_map: HashMap::new(),
        }
    }
    pub fn load_file(&mut self, file_path: &str) {
        let file = fs::File::open(file_path).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        let mut style = archive.by_name("word/styles.xml").unwrap();
        let buf = &mut String::new();
        style.read_to_string(buf).unwrap();
        self.load_style_map(buf);
        drop(style);
        let mut para = archive.by_name("word/document.xml").unwrap();
        let buf = &mut String::new();
        para.read_to_string(buf).unwrap();
        self.load_paras(buf);
    }
    pub fn get_attr(
        reader: &Reader<&[u8]>,
        event: &BytesStart,
        attr_name: &[u8],
    ) -> Option<String> {
        let mut attr_val = None;
        for attribute in event.attributes() {
            let attribute = attribute.unwrap();
            if attribute.key == attr_name {
                attr_val = Some(attribute.unescape_and_decode_value(&reader).unwrap());
            }
        }
        return attr_val;
    }
    pub fn load_style_map(&mut self, contents: &mut String) {
        let mut reader = Reader::from_str(contents);

        let mut buf = Vec::new();
        let mut path: Vec<Vec<u8>> = Vec::new();
        let mut current_attr = Attr {
            style: Style {
                bold: None,
                underline: None,
                highlight: None,
                size: None,
            },
            outline_level: None,
        };
        let mut current_style_id: Option<String> = None;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name().iter().cloned().collect();
                    path.push(name);
                    match e.name() {
                        b"w:style" => {
                            current_style_id = Self::get_attr(&reader, e, b"w:styleId");
                        }
                        _ => (),
                    };
                }
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"w:b" => {
                            let bold = Self::get_attr(&reader, e, b"w:val");
                            if bold.unwrap_or("1".to_string()) != "0" {
                                current_attr.style.bold = Some(true);
                            }
                        }
                        b"w:u" => {
                            let underline = Self::get_attr(&reader, e, b"w:val");
                            if underline.unwrap_or("single".to_string()) != "none" {
                                current_attr.style.underline = Some(true);
                            }
                        }
                        b"w:highlight" => {
                            let highlight = Self::get_attr(&reader, e, b"w:val");
                            if highlight.is_some() {
                                current_attr.style.highlight = Some(true);
                            }
                        }
                        b"w:sz" => {
                            let size = Self::get_attr(&reader, e, b"w:val");
                            if size.is_some() {
                                current_attr.style.size =
                                    Some(size.unwrap().parse::<u32>().unwrap());
                            }
                        }
                        b"w:outlineLvl" => {
                            let outline_level = Self::get_attr(&reader, e, b"w:val");
                            if outline_level.is_some() {
                                current_attr.outline_level =
                                    Some(outline_level.unwrap().parse::<u32>().unwrap());
                            }
                        }
                        b"w:basedOn" => {
                            let id = Self::get_attr(&reader, e, b"w:val");
                            if id.is_some() {
                                self.get_style_id(&mut current_attr.style, &id.unwrap());
                            }
                        }
                        _ => (),
                    };
                }
                Ok(Event::End(_e)) => {
                    let end_tag = path.pop().unwrap();
                    if end_tag == b"w:style" && path.len() == 1 {
                        self.style_map
                            .insert(current_style_id.clone().unwrap(), current_attr.clone());

                        current_attr = Attr {
                            style: Style {
                                bold: None,
                                underline: None,
                                highlight: None,
                                size: None,
                            },
                            outline_level: None,
                        };
                        current_style_id = None;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
        // for key in &self.style_map {
        //   println!("{:?}", key.0);
        //   if key.1.bold.unwrap_or(false) {
        //     println!("bold")
        //   }
        //   if key.1.underline.unwrap_or(false) {
        //     println!("underline")
        //   }
        //   if key.1.highlight.unwrap_or(false) {
        //     println!("highlight")
        //   }
        //   if key.1.size.is_some() {
        //     println!("{:?}", key.1.size.unwrap());
        //   }
        // }
    }
    pub fn load_paras(&mut self, contents: &mut String) {
        let mut reader = Reader::from_str(contents);

        let mut buf = Vec::new();
        let mut path: Vec<Vec<u8>> = Vec::new();
        let mut current_para = Para {
            index: 0,
            runs: Vec::new(),
            outline_level: None,
        };
        let mut current_run = Run {
            text: "".to_string(),
            style: Style {
                bold: None,
                underline: None,
                highlight: None,
                size: None,
            },
        };

        let mut current_para_style = Style {
            bold: None,
            underline: None,
            highlight: None,
            size: None,
        };
        self.get_style_id(&mut current_para_style, &"Normal".to_string());
        let mut current_run_style = current_para_style.clone();
        let mut current_style = &mut current_para_style;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name().iter().cloned().collect();
                    path.push(name);
                    match e.name() {
                        b"w:p" => {
                            current_style = &mut current_para_style;
                        }
                        b"w:r" => {
                            current_run_style = current_para_style.clone();
                            current_style = &mut current_run_style;
                        }
                        _ => (),
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"w:b" => {
                            let bold = Self::get_attr(&reader, e, b"w:val");
                            current_style.bold = Some(bold.unwrap_or("1".to_string()) != "0")
                        }
                        b"w:u" => {
                            let underline = Self::get_attr(&reader, e, b"w:val");
                            current_style.underline =
                                Some(underline.unwrap_or("single".to_string()) != "none")
                        }
                        b"w:highlight" => {
                            let highlight = Self::get_attr(&reader, e, b"w:val");
                            if highlight.is_some() {
                                current_style.highlight = Some(true);
                            }
                        }
                        b"w:sz" => {
                            let size = Self::get_attr(&reader, e, b"w:val");
                            if size.is_some() {
                                current_style.size = Some(size.unwrap().parse::<u32>().unwrap());
                            }
                        }
                        b"w:rStyle" => {
                            let id = Self::get_attr(&reader, e, b"w:val");
                            if id.is_some() {
                                self.get_style_id(&mut current_style, &id.unwrap());
                            }
                        }
                        b"w:pStyle" => {
                            let id = Self::get_attr(&reader, e, b"w:val");
                            if id.is_some() {
                                let id_unwrapped = id.unwrap();
                                let attr = self.style_map.get(&id_unwrapped);
                                if attr.is_some() && attr.unwrap().outline_level.is_some() {
                                    current_para.outline_level =
                                        Some(attr.unwrap().outline_level.unwrap());
                                }
                                self.get_style_id(&mut current_style, &id_unwrapped);
                            }
                        }
                        b"w:outlineLvl" => {
                            let outline_level = Self::get_attr(&reader, e, b"w:val");
                            if outline_level.is_some() {
                                current_para.outline_level =
                                    Some(outline_level.unwrap().parse::<u32>().unwrap());
                            }
                        }
                        _ => (),
                    };
                }
                Ok(Event::End(_e)) => {
                    let end_tag = path.pop().unwrap();
                    if end_tag == b"w:r" {
                        // format run text
                        // replace whitespace
                        current_run.text = current_run.text.replace("\n", "");
                        current_run.text = current_run.text.replace("\r", "");
                        current_run.text = current_run.text.replace("\t", "");

                        current_run.style = current_style.clone();
                        // if run exists
                        if current_run.text != "" {
                            // if last run style is the same
                            let last_run = &current_para.runs.last();
                            if last_run.is_some() && last_run.unwrap().style == current_run.style {
                                // remove last run and prepend text to current
                                current_run.text =
                                    last_run.unwrap().text.clone() + &current_run.text;
                                current_para.runs.pop();
                            }
                            // add new run
                            current_para.runs.push(current_run.clone());
                        }
                        current_style.bold = None;
                        current_style.underline = None;
                        current_style.highlight = None;
                        current_style.size = None;
                        current_style = &mut current_para_style;
                        current_run = Run {
                            text: "".to_string(),
                            style: Style {
                                bold: None,
                                underline: None,
                                highlight: None,
                                size: None,
                            },
                        };
                    } else if end_tag == b"w:p" {
                        current_para.index = self.paras.len();
                        self.paras.push(current_para.clone());
                        if current_para.outline_level.is_some() {
                            self.outline_paras.push(OutlinePara {
                                index: self.outline_paras.len(),
                                link: current_para.index,
                                outline_level: current_para.outline_level,
                                runs: current_para.runs,
                            });
                        }

                        current_style.bold = None;
                        current_style.underline = None;
                        current_style.highlight = None;
                        current_style.size = None;
                        self.get_style_id(&mut current_style, &"Normal".to_string());
                        current_para = Para {
                            index: 0,
                            runs: Vec::new(),
                            outline_level: None,
                        };
                    }
                }
                Ok(Event::Text(e)) => {
                    current_run.text += &e.unescape_and_decode(&reader).unwrap();
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }
    pub fn get_style_id(&mut self, style: &mut Style, style_id: &String) {
        let id_style = self.style_map.get(style_id);
        if id_style.is_some() {
            Self::mutate_style(style, &id_style.unwrap().style);
        }
    }
    // pub fn get_style(&mut self, style: &mut Style, element: &Element) {}
    pub fn mutate_style(style: &mut Style, priority_style: &Style) {
        style.bold = priority_style.bold.or(style.bold);
        style.underline = priority_style.underline.or(style.underline);
        style.highlight = priority_style.highlight.or(style.highlight);
        style.size = priority_style.size.or(style.size);
    }
}
