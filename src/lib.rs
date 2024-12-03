use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn read_input(year: u32, day: u32) -> std::io::Result<String> {
    let mut input_path = PathBuf::new();
    input_path.push("src");
    input_path.push(format!("{:04}", year));
    input_path.push(format!("{:02}", day));
    input_path.push("input.txt");

    let mut input_file = File::open(input_path)?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;

    Ok(String::from(input_string.trim()))
}
