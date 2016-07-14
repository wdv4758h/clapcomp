#[macro_use]
extern crate clap;

use clap::{YamlLoader, App, Shell};

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;    // read_to_string


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("arguments.yml");
    let arguments = App::from_yaml(yml).get_matches();

    let format = arguments.value_of("format").unwrap();
    let shell = arguments.value_of("shell").unwrap();
    let file = arguments.value_of("input").unwrap();
    let output = arguments.value_of("output").unwrap_or(".");

    ////////////////////
    // Extra Checking
    ////////////////////

    if !Path::new(output).is_dir() {
        println!("You should pass in a exsiting directory");
    }

    ////////////////////
    // Construct Application
    ////////////////////

    let shell = match shell {
        "bash" => Shell::Bash,
        _ => unreachable!(),
    };

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

    app.gen_completions(name.as_str(),  // bin name
                        shell,          // target shell
                        output);        // writing path
}
