#[macro_use]
extern crate clap;

use clap::{YamlLoader, App, Shell};

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;    // read_to_string
use std::io;


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("cli.yml");
    let arguments = App::from_yaml(yml).get_matches();

    let format = arguments.value_of("format").unwrap();
    let mut shells = arguments.values_of("shell").unwrap().collect::<Vec<_>>();
    let file = arguments.value_of("input").unwrap();
    let outfile = arguments.value_of("outfile").unwrap_or("");
    let outdir = arguments.value_of("outdir");

    shells.sort();
    shells.dedup();

    ////////////////////
    // Extra Checking
    ////////////////////

    if let Some(outdir) = outdir {
        if !Path::new(outdir).is_dir() {
            println!("You should pass in a existing directory");
            panic!();
        }
    }

    if shells.len() > 1 && !outfile.is_empty() {
        println!("You shouldn't pass in mutiple shell with specific file");
        panic!();
    }

    ////////////////////
    // Construct Application
    ////////////////////

    let shells =
        shells.iter()
              .map(|s| match s {
                    &"bash" => Shell::Bash,
                    &"fish" => Shell::Fish,
                    &"zsh" => Shell::Zsh,
                    &"powershell" => Shell::PowerShell,
                    _ => unreachable!(),
                 })
              .collect::<Vec<_>>();

    let yaml;
    let mut app = match format {
        "yaml" => {
            let mut file = File::open(file).unwrap();
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            yaml = YamlLoader::load_from_str(buffer.as_str()).unwrap()
                              .pop().unwrap();
            App::from_yaml(&yaml)
        }
        _ => unreachable!(),
    };

    let name = app.p.meta.bin_name.clone().unwrap_or(app.p.meta.name.clone());

    ////////////////////
    // Generate Completion
    ////////////////////

    for shell in shells {
        if let Some(outdir) = outdir {
            app.gen_completions(name.as_str(),  // bin name
                                shell,          // target shell
                                outdir);        // writing path
        } else if !outfile.is_empty() {
            app.gen_completions_to(name.as_str(),  // bin name
                                   shell,          // target shell
                                   &mut File::create(outfile).unwrap());
        } else {
            app.gen_completions_to(name.as_str(),  // bin name
                                   shell,          // target shell
                                   &mut io::stdout());
        }
    }
}
