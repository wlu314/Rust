use text_colorizer::*; // 
use std::env; //allows me to connect to the environemnt (terminal)
use std::fs; //file handling
use regex::Regex; //search for pattern in strings, replace text, parsing csv data

#[derive(Debug)] //allows to use standard output, to print out this struct
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!("{} - replace a string with a new strng", "Find and Replace".green());
    eprintln!("{} - <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>", "Usage".green());
}
fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} failed to read from file {}: {:?}", "Error".red().bold(), args.input_file, e);
            std::process::exit(1);
        }
    };
    let replace_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} fail to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };
    match fs::write(&args.output_file, &replace_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write from file {}: {:?}", "Error".red().bold(), args.output_file, e);
            std::process::exit(1);
        }
    };
} 
fn parse_args() -> Arguments {
    //skip the first as it's just cargo run
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!("{} wrong number of arguments given. Expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
        
    }
}
fn replace(target: &str, replace: &str, data: &str) -> Result<String, regex::Error>{
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, replace).to_string())
}
pub fn run() {
    let args = parse_args();
    read_and_write(&args);
}