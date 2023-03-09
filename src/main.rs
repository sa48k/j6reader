// Roland J-6 Reader
// cd /mnt/c/Users/super/Documents/dev/j6reader

use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    // Read args from the command line. 
    // We are looking for a valid .PRM file
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("{} (options: {})", config.file_path, config.options);

    let contents = fs::read_to_string(config.file_path).expect("Failed to read file");
    let sequence = parse_sequence(&contents);

    println!("Tempo: {}", sequence.tempo / 100.0);
    println!("Beat: {}", sequence.beat);
    println!("Filter: {}", sequence.filter);
    println!("Measures: {}", sequence.meas);

    // TODO: get pattern number + bank number from filename
    let full_song: Vec<String> = convert_bars_to_strings(sequence.bars);
    let x = (sequence.meas * 4.0) as usize;
    let song = &full_song[0..x];
    // dbg!(&song);
    for (idx, note) in song.iter().enumerate() {
        print!("{} ", note);
        if (idx+1) % 4 == 0 {
            println!();
        }

    }
}

struct Config {
    options: String,
    file_path: String,
}

struct SequenceOptions {
    beat: f32,
    meas: f32,              // number of measures (bars)
    tempo: f32,
    filter: f32,
    bars: [[i32; 4]; 64]    // contains raw note values from the J-6, e.g. [38, 53, 55, 60] x64
}

fn generate_notes_hashmap() -> HashMap<usize, String> {
    let notes_string = "C.C#.D.D#.E.F.F#.G.G#.A.A#.B";
    let split_notes: Vec<String> = notes_string.split(".").map(|s| s.to_string()).collect();
    let mut note_lookup = HashMap::new();

    for x in 0..95 {
        note_lookup.insert(x, split_notes[x % 12].clone());
    }
    
    note_lookup
}

fn convert_bars_to_strings(bars: [[i32; 4]; 64]) -> Vec<String> {
    let mut song: Vec<String> = vec![];
    let note_lookup = generate_notes_hashmap();

    for bar in &bars {
        for value in bar {
            let value: i32 = *value;
            // check that value isn't -1. if it is, return '--' (no note)
            if value == -1 {
                song.push("--".to_string());
            } else {
                let mut note: String = note_lookup[&((value as usize) % 12)].clone();   // gets the note (C, F, G, G#, etc.) as String
                let octave: f32 = (value as f32 / 12.0).floor();                        // calculate the octave
                note.push_str(&(octave.to_string()));                                   // concat the strings
                song.push(note);
            }
        }
    }
    song
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

fn read_bars(contents: &str) -> [[i32; 4]; 64] {
    // we'll read ALL 64 bars, even if MEAS < 64
    // this way we can implement full sequence editing functionality
    // maybe
    let mut bar: [i32; 4] = [0; 4];
    let mut bars: [[i32; 4]; 64] = [[0; 4]; 64];
    for line in contents.lines() {
        if line.starts_with("BAR") {
            // get the bar number
            let bar_number: i32 = line[4..6].trim().parse::<i32>().unwrap(); // panic if not a valid integer
            let mut count = 0;
            for token in line.split(' ') {
                if token.starts_with("NOTE") {
                    let note: i32 = token
                        .split("=")
                        .last()
                        .unwrap()
                        .trim()
                        .parse::<i32>()
                        .unwrap();
                    bar[count] = note;
                    count += 1;
                }
            }
            bars[(bar_number-1) as usize] = bar;
        }
    }

    bars
}

fn parse_sequence(contents: &str) -> SequenceOptions {
    let tempo = read_setting(&contents, "TEMPO");
    let beat = read_setting(&contents, "BEAT");
    let meas = read_setting(&contents, "MEAS");
    let filter = read_setting(&contents, "FILTER");
    
    let bars = read_bars(&contents);

    SequenceOptions {
        tempo,
        beat,
        filter,
        meas,
        bars,
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
