use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub fn load_commands(file_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let reader = match load_file(&file_path) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Can't load {:?}", file_path);
            Err("Error loading commands")?
        }
    };

    let mut entries = Vec::new();
    for line in reader.lines() {
        let mut line = line?;
        line = line.trim().to_owned();

        let ignore_line = line.starts_with('/') || line.is_empty();
        if ignore_line {
            continue;
        }

        // removes extra white spaces between words
        let words: Vec<_> = line.split_whitespace().collect();
        line = words.join(" ");

        // removes comments
        let words: Vec<_> = line.split(" //").collect();
        line = words[0].to_owned();

        // builds commands
        let command_str = format!("-{line}");

        entries.push(command_str);
    }

    Ok(entries)
}

pub fn load_cfg(file_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let reader = match load_file(&file_path) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Can't load {:?}", file_path);
            Err("Error loading .cfg")?
        }
    };

    let mut entries = Vec::new();
    for line in reader.lines() {
        let mut line = line?;
        line = line.trim().to_owned();

        let ignore_line = line.starts_with('/') || line.is_empty();
        if ignore_line {
            continue;
        }

        // removes extra white spaces between words
        let words: Vec<_> = line.split_whitespace().collect();
        line = words.join(" ");

        // removes comments
        let words: Vec<_> = line.split(" //").collect();
        line = words[0].to_owned();

        entries.push(line);
    }

    Ok(entries)
}

pub fn load_file(file_path: &Path) -> Result<BufReader<File>, Box<(dyn Error + 'static)>> {
    if !file_path.exists() {
        Err("Error loading file")?;
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}
