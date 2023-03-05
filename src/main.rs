use std::env;
use std::fs;

fn main() {
    // Read args from the command line. We are looking for a valid .PRM file
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("{} (options: {})", config.file_path, config.options);

    let contents = fs::read_to_string(config.file_path)
        .expect("Failed to read file");
    
    let sequence = parse_sequence(&contents);
    println!("Sequence has a tempo of {}", sequence.tempo/100);
}

struct Config {
    options: String,
    file_path: String,
}

struct SequenceOptions {
    // beat: i16,
    // transpose: i16,
    tempo: i32,
    // filter: i16,
    // variation: i16,
    // style_sw: i16,
}

// bars: [u8; 4]

fn parse_sequence(contents: &String) -> SequenceOptions {
    let mut tempo: i32 = 0;
    for line in contents.lines() {
        if line.contains("TEMPO") {
            // split at the equals, grab the string to the right, trim it, parse it to an integer
            tempo = line.split("=").nth(1).unwrap().trim().parse::<i32>().unwrap();
        }
    }
    SequenceOptions { tempo }
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let file_path = args[1].clone();
    let options = args[2].clone();

    Config { file_path, options }
}


