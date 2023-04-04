use clap::Parser;
use std::{fs, io::Write, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    match args.file {
        Some(_file) => generate_cpp_file(_file),
        None => compile_file(),
    }
}

fn compile_file() {
    let entries: Vec<fs::DirEntry> = fs::read_dir(".")
        .expect("couldn't read the directory")
        .filter(|f| {
            f.as_ref()
                .expect("Failed to filter file")
                .metadata()
                .expect("failed to read the file metadata")
                .is_file()
        })
        .flatten()
        .collect();
    let file = entries.iter().max_by_key(|f| {
        f.metadata()
            .expect("failed to read the metadata")
            .modified()
            .expect("Failed to read the modified date")
    });
    if let Some(f) = file {
        let file_name_raw = f.file_name();
        let file_name = file_name_raw
            .to_str()
            .expect("Failed to convert the osstr to str")
            .trim_matches('"');
        println!("\nCompiling {} with g++", file_name);
        let compile_output = Command::new("g++")
            .arg(format!("{}", file_name))
            .arg("-o")
            .arg(format!("{}", file_name.trim_end_matches(".cpp")))
            .output();
        match compile_output {
            Ok(_) => {
                println!("\nCompiled successfully")
            }
            Err(e) => println!("{e}"),
        }
        println!("\nRunning {}", file_name);
        let run_output = Command::new(format!("./{}", file_name.trim_end_matches(".cpp"))).status();
        match run_output {
            Ok(o) => {
                if let Some(code) = o.code() {
                    println!("\n\nProgram exited with status {}", code)
                }
            }
            Err(e) => println!("{e}"),
        }
    } else {
        println!("Failed to compile the file");
    }
}

fn generate_cpp_file(file: String) {
    let file_name = format!("{}.cpp", file);
    println!("{file_name}");
    let fmt_text = format!(
        "
/*
ID: saradga1
TASK: {}
LANG: C++                 
*/
#include<bits/stdc++.h>
#include<iostream.h>
using namespace std;

int main(){{
    return 0;
}}
    ",
        file
    );
    let mut fls = fs::File::create(&file_name).expect("Error occured while creating the file");
    //if let Err(e) = flsfmt_text) {};
    if let Err(e) = write!(fls, "{}", fmt_text) {
        println!("{} occured", e);
        return;
    }
}
