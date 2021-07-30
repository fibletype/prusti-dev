use execute::Execute;
use prusti_launch::get_rust_toolchain_channel;
use std::process::{Command, Stdio};

use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
// #[structopt(name = "basic")]
#[structopt(setting = AppSettings::InferSubcommands)]
struct Opts {
    tool: Tool,
}

#[derive(StructOpt, Debug)]
enum Tool {
    #[structopt(alias = "rustc")]
    Rustc,
    #[structopt(alias = "cargo")]
    Cargo,
}

impl Tool {
    fn name(&self) -> &'static str {
        match self {
            Tool::Rustc => "prusti-rustc",
            Tool::Cargo => "cargo-prusti",
        }
    }

    fn args(&self, args: &[String]) -> Vec<String> {
        let mut result = match self {
            // Tool::Rustc => vec!["--error-format=json", "--crate-type=lib", "--edition=2018"],
            Tool::Rustc => vec![
                "--error-format=json".to_string(),
                "--crate-type=lib".to_string(),
            ],
            Tool::Cargo => vec![],
        };

        result.extend(args.into_iter().cloned());
        result
    }
}

impl std::str::FromStr for Tool {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rustc" => Ok(Tool::Rustc),
            "cargo" => Ok(Tool::Cargo),
            _ => Err(format!("unsupported tool '{}'", s)),
        }
    }
}

pub fn process() {
    let args = std::env::args().collect::<Vec<String>>();
    let opt = Opts::from_iter(args[..2].iter());
    let (processed, args) = args.split_at(2);

    let args = opt.tool.args(&args);
    let name = opt.tool.name();

    println!("ARGS: {:?}", args);
    println!("PWD: {:?}", std::env::current_dir());
    let mut path = process_path::get_executable_path()
        .unwrap_or(std::env::current_exe().expect("The process path could not be determined"));
    // pop the self name:
    path.pop();
    // add tool name:
    path.push(name);
    if cfg!(windows) {
        path.set_extension("exe");
    }

    println!("TOOL: {:?}", &path);

    let mut command = Command::new(path);
    args.into_iter().for_each(|arg| {
        command.arg(arg);
    });
    // command.arg("--error-format=json");
    // // command.arg("/path/to/media-file");
    // // command.arg("/path/to/output-file");

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = command.execute_output().unwrap();

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else {
        eprintln!("Interrupted!");
    }

    println!("STDOUT:\n{}", String::from_utf8(output.stdout).unwrap());
    println!("STDERR:\n{}", String::from_utf8(output.stderr).unwrap());


	// find errors json:
	// Verification of 4 items
	// ...
	// Verification failed
}
