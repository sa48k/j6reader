# Roland J-6 Reader

Read sequence data from the .PRM files stored on a Roland J-6 synthesiser. 

To get these files, connect your J-6 to your PC using USB. Hold the Play button while switching it on, wait for it to mount, then open the BACKUP directory - your sequences are contained within these .PRM files. 

Only tested on Ubuntu but probably works on other OSs

## Usage

Run this j6reader tool, passing the path to a .PRM file as the first argument, and anything for a second argument (to implement in future: different options such as output-to-MIDI, output-to-JSON). For now it only outputs to the terminal.
`./j6reader <filepath>/J6_PTNxx.PRM read`

    Running `target/debug/j6reader BACKUP/J6_PTN22.PRM opts`
    Tempo: 132.5
    Beat: 6
    Filter: 18
    Measures: 8
    Pattern bank: 3-6

    1       2       3       4       5       6       7       8

    C#4     D4      C4      E4      G#3     G#3     C#4     C4
    C5      F5      G4      C5      C5      C5      C5      G4
    D#5     A4      D5      D5      D5      D5      D#5     D5
    F5      C5      E5      G5      F5      F5      F5      E5

## Development / Reflection

My first Rust project. Written in 100% Rust with no dependencies. Ownership now makes a bit more sense. At least one silly design mistake: the numerical note values from the J-6 are encoded as `[[i32; 4]; 64]` (i.e. 64 bars, each with four i32 values), which makes sense. However, after converting these values to actual notes ("C4", "F#4", etc.), the result is stored as a 1D Vector, though it would make more sense as a Vector of `[[String], 4]`s.
