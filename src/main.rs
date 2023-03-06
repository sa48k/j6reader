// Roland J-6 Reader
// cd /mnt/c/Users/super/Documents/dev/j6reader

use std::env;
use std::fs;

fn main() {
    // Read args from the command line. We are looking for a valid .PRM file
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("{} (options: {})", config.file_path, config.options);

    let contents = fs::read_to_string(config.file_path).expect("Failed to read file");

    let sequence = parse_sequence(&contents);
    println!("Tempo: {}", sequence.tempo / 100.0);
    println!("Beat: {}", sequence.beat);
    println!("Filter: {}", sequence.filter);
    // dbg!(sequence.bar);
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
    bar: [i32; 4], // bars: [[u8; 4]; 64]
}

fn read_setting(contents: &str, setting: &str) -> f32 {
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

fn read_bars(contents: &str) -> [i32; 4] {
    let mut bar: [i32; 4] = [0; 4];
    for line in contents.lines() {
        if line.starts_with("BAR") {
            let mut count = 0;
            for token in line.split(' ') {
                if token.starts_with("NOTE") {
                    let note: i32 = token.split("=").last().unwrap().trim().parse::<i32>().unwrap();
                    bar[count] = note;
                    count += 1;
                }
            }
            dbg!(bar);
        }
    }

    bar
}

fn parse_sequence(contents: &str) -> SequenceOptions {
    let tempo = read_setting(&contents, "TEMPO");
    let beat = read_setting(&contents, "BEAT");
    let filter = read_setting(&contents, "FILTER");

    let bar = read_bars(&contents);
    SequenceOptions {
        tempo,
        beat,
        filter,
        bar,
    }
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let file_path = args[1].clone();
    let options = args[2].clone();

    Config { file_path, options }
}
