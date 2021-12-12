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

fn edit_file(file: &str) {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| prompt("Please input your editor: "));
    let file_path = "/tmp/tmp.txt";
    let mut file_contents = String::new();
    File::open(file).unwrap().read_to_string(&mut file_contents).unwrap();
    let mut editable_file = File::create(&file_path).expect("failed to create tmp file");
    writeln!(editable_file, "{}", file_contents).unwrap();
    
    std::process::Command::new(editor)
        .arg(file_path)
        .status()
        .expect("failed to spawn editor");

    let mut modified_file_contents = String::new();
    let mut modified_editable_file = File::open(&file_path).unwrap();
    modified_editable_file.read_to_string(&mut modified_file_contents).unwrap();

    writeln!(File::create(file).unwrap(), "{}", modified_file_contents).unwrap();
}

fn view_returned(file: &str) -> String {
    let usable_file = File::open(file).unwrap();
    let mut bufreader = BufReader::new(usable_file);
    let mut contents = String::new();
    bufreader.read_to_string(&mut contents).unwrap();
    contents
}

fn list_all_files(dir: &str) -> Vec<(&str, String)> {
    let mut files = Vec::new();
    if std::path::Path::new(dir).exists() {
        let file_list = std::fs::read_dir(dir).unwrap();
        for file in file_list {
            let entry = file.unwrap().path();
            let file_type = if entry.is_dir() {
                "dir"
            } else {
                "file"
            };
            let formatted_entry = entry.to_str().unwrap().replace("./", "");
            files.push((file_type, formatted_entry));
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
    for (idx, (ftype, formatted_file)) in files.iter().enumerate() {
        // file_index, file_type, file_name
        println!("{0: <} {1: <4} {2: <}", idx, ftype, formatted_file);
    }
    let input: usize = prompt("> ").parse().unwrap();
    let file = &files.get(input).unwrap().1;
    let op = prompt(format!("What would you like to do with '{}'?\nYou can use 'e' to edit it and 'v' to view it. ", file).as_str());
    match op.as_str() {
        "e" | "edit" => edit_file(file),
        "v" | "view" => println!("{}", view_returned(file)),
        _ => println!("'{}' is an unsupported operation!", op)
    }
}
