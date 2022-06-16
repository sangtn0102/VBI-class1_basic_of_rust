use std::{fs, io, path::Path};

fn main() {
    //input from keyboard
    println!("Please enter a string: ");
    let buf_input = input_string();
    buf_input.to_string().pop();

    //read input file
    let path = Path::new("./data/1-s2.0-S0960982203005347-mmc6.txt");
    let contents_file = read_file(path);

    count_text(contents_file, &buf_input)
}

fn read_file(filename: &Path) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents
}

fn input_string() -> String {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string
}

fn count_text(input: String, slice: &str) {
    let count = slice.matches(&input.as_str().trim()).count();
    println!("You have {} in slice: {}", count, slice);
}
