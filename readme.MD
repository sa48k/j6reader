# Roland J-6 Reader

Read sequence data from the .PRM files stored on a Roland J-6 synthesiser. 

To get these files, connect your J-6 to your PC using USB. Hold the Play button while switching it on, wait for it to mount, then open the BACKUP directory.

## Usage

`./j6reader J6_PTNxx.PRM read`

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