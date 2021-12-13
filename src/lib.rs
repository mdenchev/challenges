use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    read_lines_iter(filename).into_iter().collect()
}

pub fn read_lines_iter<P>(filename: P) -> impl IntoIterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap())
}
