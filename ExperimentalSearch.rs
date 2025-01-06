use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    if let Some(search_term) = env::args().skip(1).collect::<Vec<String>>().join(" ").split_whitespace().next() {
        if !is_command_available("fzf") {
            eprintln!("fzf is not installed. Please install it to use this script.");
            return;
        }

        let search_term = search_term.replace(" ", "+");
        let local_file = search_local_files(&search_term);

        if let Some(file) = local_file {
            println!("Opening local file: {}", file);
            if Command::new("xdg-open").arg(&file).status().is_err() {
                eprintln!("Failed to open the local file.");
            }
        } else {
            println!("Opening search engine for: {}", search_term);
            let url = format!("https://watch.qtchaos.de/browse/{}", search_term);
            if Command::new("xdg-open").arg(&url).status().is_err() {
                eprintln!("Failed to open the URL in the default web browser.");
            }
        }
    } else {
        eprintln!("Please provide a search term.");
    }
}

fn is_command_available(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {}", command))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn search_local_files(search_term: &str) -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("find . -type f -iregex '.*\\.(mp4|mkv|avi|mov|flv|webm|m4v|wmv|mpg|mpeg|3gp|mts|m2ts)' | fzf --query=\"{}\" --preview=\"file {{}}\" --preview-window=right:70% --exact", search_term))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        let file = result.trim();
        if Path::new(file).exists() {
            return Some(file.to_string());
        }
    }
    None
}
