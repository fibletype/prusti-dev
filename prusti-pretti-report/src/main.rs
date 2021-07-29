// #![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use pulldown_cmark::{Parser, Options, html};
use serde_json::json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Tests {
    ok_ : bool,
    func : String,
    line : String, 
    symbl : u16
}

#[derive(Serialize, Deserialize)]
struct Spans {
    line_start : u64,
    line_end : u64
}

#[derive(Serialize, Deserialize)]
struct Children {
    message : String, 
    spans : Vec<Spans>
}

#[derive(Serialize, Deserialize)]
struct VerificationResult {
    message : String,
    children : Vec<Children>
}

fn init_template (prog : &String, status : bool, start : u64, end : u64) -> String {
    
    if status {
        //Программа ++ prog ++ успешно верифицирована. В этом файле вы можете увидеть доказательства соответствия спецификации
        format!("Program () {} has been verified sucsesfully", &prog)
    }
    else {

        format!("Program () {} don't satisfies specification. look at line {}-{}", &prog, start, end)
        /* .push_str(prog) */
        //Программа ++ prog ++ не соответствует спецификации. 
        //Обратите внимание на ++ строки и пре/пост конидшоны.
    }
}

fn main() -> Result<(), std::io::Error> {

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let json = File::open("eg.json");
    let mut contents = String::new() ;
    let json = json.map(|mut f|{
        f.read_to_string(&mut contents)
    }).unwrap();
    let info : VerificationResult = serde_json::from_str(&contents)?;
    

    println!("{}", info.children[1].message);

    let prog_name = "example".to_string();

    let status = !info.message.contains("error");
    println!("{}", status);
    let init_str = init_template(&prog_name, status, info.children[1].spans[0].line_start,info.children[1].spans[0].line_end);
    let parser = Parser::new_ext(&init_str, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let mut file = File::create("ex.html");
    if let Ok(mut file) = file {
        file.write(&html_output.as_bytes());
    }






    let data = r#"
    {
        "ok_": true,
        "func": "Alestra",
        "line": "12",
        "symbl": 13,
        "xyi": "sdfsdf"
    }"#;
    
    let test : Tests = serde_json::from_str(data)?;
    // Convert to a string of JSON and print it out
    println!("Poshel naxyi {}", test.ok_);

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
   /*  let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let status = true; //need for prusty
    let prog_name = "example".to_string(); //need form prog name
    let init_str = init_template(&prog_name, status);
    let parser = Parser::new_ext(&init_str, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser); */
    //html::write_html(, iter)
    // Check that the output is what we expected.
/*     let expected_html = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    assert_eq!(expected_html, &html_output); */

    
    Ok(())
}



// Напиать функцию, которая принемает ссылку на слайс от вектора и что-нибудь с ним делает.
// Передать мутабельную ссылку и два раза из мейн вызвать эту функцию.
// Что будет, если он не может создать контрпример
// Create dot
// обработать ошибку при открытии файла