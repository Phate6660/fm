use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().to_string()
}

fn view_returned(file: &str) -> String {
    let usable_file = File::open(file).unwrap();
    let mut bufreader = BufReader::new(usable_file);
    let mut contents = String::new();
    bufreader.read_to_string(&mut contents).unwrap();
    contents
}

fn list_all_files(dir: &str) -> HashMap<usize, (&str, String)> {
    let mut files = HashMap::new();
    if std::path::Path::new(dir).exists() {
        let file_list = std::fs::read_dir(dir).unwrap();
        for (idx, file) in file_list.enumerate() {
            let entry = file.unwrap().path();
            let file_type = if entry.is_dir() {
                "dir"
            } else {
                "file"
            };
            let formatted_entry = entry.to_str().unwrap().replace("./", "");
            files.insert(idx, (file_type, formatted_entry));
        }
    }
    files
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cwd = String::from(".");
    let dir = args.get(1).unwrap_or(&cwd);
    let files = list_all_files(dir);
    println!("Please select a file via number.");
    let sorted_files: Vec<(_, _)> = files.iter().map(|(_id, (ft, fb))| { (ft, fb) }).collect();
    for (idx, (ftype, formatted_file)) in sorted_files.iter().enumerate() {
        // file_index, file_type, file_name
        println!("{0: <} {1: <4} {2: <}", idx, ftype, formatted_file);
    }
    let input: usize = prompt("> ").parse().unwrap();
    let file = &files.get(&input).unwrap().1;
    let op = prompt(&format!("What would you like to do with '{}'?\nYou can use 'v' to view it. ", file).as_str());
    match op.as_str() {
        "v" | "view" => {
            let output = view_returned(&file);
            println!("{}", output);
        },
        _ => println!("'{}' is an unsupported operation!", op)
    }
}