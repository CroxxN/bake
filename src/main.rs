use clap::Parser;
use std::{
    fs,
    io::Write,
    process::Command,
    time::{self, Duration, UNIX_EPOCH},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: Option<String>,
    #[arg(short = 't')]
    template: bool,
}

fn main() {
    let args = Args::parse();
    if args.template {
        match args.file {
            Some(_file) => generice_cpp(_file),
            None => {
                println!("No file name supplied!");
                return;
            }
        }
    } else {
        match args.file {
            Some(_file) => generate_usaco_template(_file),
            None => compile_file(),
        }
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
            .arg("-std=c++17")
            .arg("-Wall")
            .status();
        match compile_output {
            Ok(o) => {
                if let Some(code) = o.code() {
                    if code != 0 {
                        println!("\nFailed to compile. Status {} returned by g++\n", code);
                        return;
                    } else {
                        println!("\nCompiled successfully with status {}\n", code)
                    }
                } else {
                    println!("\nCompiled successfully\n",)
                }
            }
            Err(e) => {
                println!("{e}");
                return;
            }
        }
        println!("\nRunning {}\n", file_name);
        let run_output = Command::new(format!("./{}", file_name.trim_end_matches(".cpp"))).status();
        match run_output {
            Ok(o) => {
                if let Some(code) = o.code() {
                    if code != 0 {
                        println!("Failed to run. Exited with status: {}", code);
                        return;
                    }
                    println!("\n\nProgram exited with status {}", code)
                }
            }
            Err(e) => println!("{e}"),
        }
    } else {
        println!("Failed to compile the file");
    }
}

fn generice_cpp(file: String) {
    let file_name = format!("{}.cpp", file);
    let curr_time = match time::SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => dur,
        Err(_) => Duration::new(1, 0),
    };
    println!("{file_name}");
    let fmt_text = format!(
        "
/*
User: crox-x
LANG: C++                 
Time: {}
*/
#include<bits/stdc++.h>
#include<iostream>
using namespace std;

int main(){{
    ios::sync_with_stdio(false);
    cin.tie(0);
    return 0;
}}
    ",
        curr_time.as_secs_f32()
    );
    let mut fls = fs::File::create(&file_name).expect("Error occured while creating the file");
    //if let Err(e) = flsfmt_text) {};
    if let Err(e) = write!(fls, "{}", fmt_text) {
        println!("{} occured", e);
        return;
    }
}

fn generate_usaco_template(file: String) {
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
#include<iostream>
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
