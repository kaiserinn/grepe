use anyhow::{ Context, Result };
use inline_colorization::*;
use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about="grepe your mom", long_about = None)]
pub struct Config {
    query: String,
    file_path: std::path::PathBuf,

    #[arg(short, long)]
    ignore_case: bool,
}

pub fn run(config: Config) -> Result<()> {
    let contents = fs::read_to_string(&config.file_path)
        .with_context(|| format!("failed to read contents from {:?}", config.file_path))?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    let colored_string = &format!("{color_blue}{}{color_reset}", &config.query);
    let colored_string_lower = &format!("{color_blue}{}{color_reset}", &config.query.to_lowercase());

    for line in result {
        let mut temp = line.replace(&config.query, colored_string);
        temp = temp.replace(&config.query.to_lowercase(), colored_string_lower);
        println!("{temp}");
    }

    Ok(())
}

pub fn search(query: &str, contents: &str) -> Vec<String> {
    let mut result = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            result.push(format!("{color_green}{}{color_reset}: {}", i + 1, line));
        }
    }

    result
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    let mut result = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(format!("{color_green}{}{color_reset}: {}", i + 1, line));
        }
    }

    result
}
