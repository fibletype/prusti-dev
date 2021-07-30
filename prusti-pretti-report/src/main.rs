// #![allow(unused)]
use pulldown_cmark::{html, Options, Parser};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::{fs::File, io::prelude::*};

mod process;

#[derive(Serialize, Deserialize)]
struct Text {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Spans {
    file_name: String,
    column_start: u16,
    column_end: u16,
    text: Vec<Text>,
    line_start: u64,
    line_end: u64,
}

#[derive(Serialize, Deserialize)]
struct Children {
    message: String,
    spans: Vec<Spans>,
}

#[derive(Serialize, Deserialize)]
struct VerificationResult {
    message: String,
    spans: Vec<Spans>,
    children: Vec<Children>,
}

fn init_template(status: bool) -> String {
    if status {
        format!("Project has been verified successfully.")
    } else {
        format!("Program doesn't satisfy specification.")
    }
}

fn main() -> Result<(), std::io::Error> {
    process::process();
    return Ok(());

    //  let json = File::open("prusti-pretti-report/tests/eg.json");
    //  let mut contents = String::new();
    //  let _ = json.map(|mut f| f.read_to_string(&mut contents)).unwrap();

    //  process_json(contents)?;
    Ok(())
}

fn process_json(json: String) -> Result<(), std::io::Error> {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_STRIKETHROUGH);

    let info: VerificationResult = serde_json::from_str(&json)?;
    let prog_name = "example".to_string();
    if json.trim().is_empty() {
        let mut init_str = init_template(true);
        let parser = Parser::new_ext(&init_str, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        let mut file = File::create("ex.html");
        if let Ok(mut file) = file {
            file.write(&html_output.as_bytes());
        }
        std::process::exit(1)
    }

    let mut init_str = init_template(false);

    for i in info.children.iter() {
        if i.message.contains("error") {
            for j in i.spans.iter() {
                init_str += &format!(
                    "\n\n in {} \n\n form line {} to line {} ...",
                    j.file_name, j.line_start, j.line_end
                );
                for k in j.text.iter() {
                    init_str += &format!("\n\n {} ", k.text);
                }
            }
        } else if i.message.contains("counterexample") {
            init_str += &format!("\n\n {}", i.message.to_string());
        }
    }
    init_str += &format!("\n\n {}", info.spans[0].text[0].text); //Very Bad
    let parser = Parser::new_ext(&init_str, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let mut file = File::create("ex.html");
    if let Ok(mut file) = file {
        file.write(&html_output.as_bytes());
    }

    /* let tmt = include_str!("tmt.html");
    tmt.replace("!!!!!REPLACE!!!!!", html_con); */

    Ok(())
}

// Что будет, если он не может создать контрпример
// Create dot
// обработать ошибку при открытии файла

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_should_not_crash() {
        let contents = include_str!("../tests/eg.json");
        process_json(contents.to_string()).expect("parse json & render");
    }
}
