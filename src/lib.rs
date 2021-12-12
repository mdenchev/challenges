use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> impl IntoIterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
}
