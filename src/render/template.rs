use super::*;
use crate::render;
use render::interpreter;
use std::path::Path;
use futures_util::StreamExt;
use warp::path::{end, peek};

#[derive(Clone)]
pub struct HTMLTemplate {
    body: String,
    error_stack: Vec<Error>
}

impl HTMLTemplate {
    pub fn new(path: &str) -> Self {
        let template_path = Path::new(path);

        let dst_path = Path::new("./public/templates/").join(template_path);
        if !dst_path.exists() {
            eprintln!("template does not exist: {}", path);
            return Self::default();
        }

        let Ok(raw_template) = std::fs::read_to_string(dst_path.clone()) else {
            eprintln!("unable to read template file: {:?}", dst_path);
            return Self::default();
        };


        Self { body: raw_template, error_stack: Vec::new() }
    }
}

impl HTMLTemplate {
    fn formatted_output(mut self) -> Result<String, Error> {
        let out = &self.body;

        let mut tags = Vec::new();

        for (i, curr_char) in out.char_indices() {
            match curr_char {
                '{' => {
                    let Some(peek_char) = out.chars().nth(i + 1) else {
                        eprintln!("unable to peek next char");
                        continue
                    };
                    //println!("i: {} char: {} peek: {}", i, curr_char, peek_char);
                    if peek_char == '{' {
                        tags.push(i);
                    }
                }
                _ => continue
            }
        }



        let mut end_tags = Vec::new();
        for tag in tags.iter() {
            let mut iter = out.char_indices();
            if let Err(i) = iter.advance_by(*tag) {
                eprintln!("failed to move to char at {}", i);
                continue
            }
            for (i, curr_char) in iter {
                //println!("xyx: {} {}", i, curr_char);
                match curr_char {
                    '}' => {
                        let Some(peek_char) = out.chars().nth(i + 1) else {
                            eprintln!("unable to peek next char");
                            continue
                        };

                        if peek_char == '}' {
                            end_tags.push(i+1);
                            break
                        }
                    }
                    _ => continue
                }

            }
        }

        if tags.len() != end_tags.len() {
            return Err(Error::HTML(String::from("Invalid parse state occurred, start tags length does not match end tags length.")));
        }

        let mut template_interpreter  = interpreter::Template::new();
        for (i, tag) in tags.iter().enumerate() {
            let Some(end_tag) = end_tags.get(i) else {
                return Err(Error::HTML(format!("Internal error: could not access corresponding end_tag at index {}", i)))
            };
            match self.do_interpreter(&template_interpreter, *tag, *end_tag) {
                Ok(result) => println!("interpreted: {:?}", result),
                Err(e) => self.error_stack.push(Error::Interpreter(e))
            }
        }

        println!("tags: {:?}\nend: {:?}", tags, end_tags);
        Ok(self.body)
    }

    fn do_interpreter(&self, mut intptr: &interpreter::Template, start: usize, end: usize) -> Result<interpreter::Literal, interpreter::Error> {
        let input = &self.body[start..end];
        intptr.interpret(input)
    }

}

impl Item<String> for HTMLTemplate {
    fn compile(self) -> Result<String, Error> {
        self.formatted_output()
    }
}

impl Default for HTMLTemplate {
    fn default() -> Self {
        Self { body: String::from(""), error_stack: Vec::new() }
    }
}