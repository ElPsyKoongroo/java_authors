use std::{
    io::{BufRead, Write},
    path::PathBuf,
    sync::RwLock,
};

use clap::Parser;
static MAX_DEPTH: u32 = 6;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    author: String,

    #[arg(short, long, default_value = ".")]
    project_dir: String,
}

static AUTHOR: RwLock<String> = RwLock::new(String::new());

fn write_file_buffer(file_path: PathBuf, content: &str) {
    let mut writter = std::io::BufWriter::new(std::fs::File::create(&file_path).unwrap());
    match writter.write_all(content.as_bytes()) {
        Ok(_) => println!("Author applied to {:?}", file_path),
        Err(_) => {}
    }
}

fn add_author(file_buffer: &mut String) {
    *file_buffer = format!(
"/**
*
* @author {}
*/\n",
        *AUTHOR.try_read().unwrap()
    ) + file_buffer;
}

fn check_author(file_path: PathBuf) {
    let mut reader = std::io::BufReader::new(std::fs::File::open(&file_path).unwrap());

    let mut file_buffer = "".to_owned();
    let mut buffer = "".to_owned();
    let mut author_added = false;
    while reader.read_line(&mut buffer).unwrap() != 0 {
        if buffer.contains("@author") {
            file_buffer.push_str(&format!("* @author {}\n", *AUTHOR.try_read().unwrap()));
            author_added = true;
        } else {
            file_buffer.push_str(&buffer)
        }
        buffer.clear()
    }

    if !author_added {
        add_author(&mut file_buffer)
    }
 
    write_file_buffer(file_path, &file_buffer);
}

fn scan_folder(actual_path: PathBuf, actual_depth: u32) {
    if actual_depth > MAX_DEPTH {
        return;
    }

    for entry in actual_path.read_dir().unwrap() {
        let Ok(file) = entry else { continue };

        if file.file_type().unwrap().is_file()
            && file.file_name().to_str().unwrap().contains(".java")
        {
            check_author(file.path());
        } else if file.file_type().unwrap().is_dir() {
            scan_folder(file.path(), actual_depth + 1);
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("Author: {}", args.author);
    let path = PathBuf::from(args.project_dir);

    *AUTHOR.try_write().unwrap() = args.author.clone();

    scan_folder(path, 0)
}
