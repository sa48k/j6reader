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
    println!("Tempo: {}", sequence.tempo/100.0);
    println!("Beat: {}", sequence.beat);
    println!("Filter: {}", sequence.filter);
}

struct Config {
    options: String,
    file_path: String,
}
 
struct SequenceOptions {
    beat: f32,
    // transpose: i16,
    tempo: f32,
    filter: f32,
    // variation: i16,
    // style_sw: i16,
}

// bars: [u8; 4]

fn find_setting(contents: &str, setting: &str) -> f32 {
    // search the file contents for an occurence of a setting
    // e.g. BEAT, GENRE, or TRANSPOSE
    // return its associated value as an f32 to be stored in the struct
    
    contents
        .lines()
        .find(|line| line.starts_with(setting))
        .unwrap()
        .split('=')
        .last()
        .unwrap()
        .trim()
        .parse::<f32>()
        .unwrap()
}

fn parse_sequence(contents: &String) -> SequenceOptions {
    let tempo = find_setting(contents, "TEMPO");
    let beat = find_setting(contents, "BEAT");
    let filter = find_setting(contents, "FILTER");

    SequenceOptions { tempo, beat, filter }
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let file_path = args[1].clone();
    let options = args[2].clone();

    Config { file_path, options }
}


