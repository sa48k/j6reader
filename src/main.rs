use std::env;
use std::fs;

fn main() {
    // Read args from the command line. We are looking for a valid .PRM file
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Opening {}...", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");

    for line in contents.lines() {
        println!("{}", line);
    }
}
