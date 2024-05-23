#![feature(const_trait_impl)]
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, Duration};
use std::sync::mpsc;
use std::thread;
use std::str::from_utf8;

use chrono::Local;
use inline_colorization::{
    color_red as WARN,
    color_magenta as DEBUG, 
};
const RESET: &'static str = "\u{1b}[39m\u{1b}[49m";

fn now() -> String {Local::now().format("%H:%M:%S").to_string()}

const TEX_MAIN : &'static str = "main.tex";

fn compile_tex(dir: &str) {
    let output = Command::new("hilatex")
                                        .arg(Path::new(dir).join(TEX_MAIN))
                                        // .arg("-output-directory")
                                        // .arg(dir)
                                        .output().expect("failed to execute process");
    let e = from_utf8(&output.stdout).unwrap();
    // println!("{DEBUG}Debug:{RESET} Output: {}", e);
    for error_line in e.lines()
                        .filter(|line| line.starts_with("!")) {
        println!("{WARN}{}{RESET}", error_line);
    }
}

fn watch_hnt_files(dir: &str) {
    let (tx, rx) = mpsc::channel();
    let mut hnt_time = SystemTime::now();

    let dir_entries = fs::read_dir(Path::new(dir)).unwrap();

    for path in dir_entries.map(|e| e.unwrap().path())
                                    .filter(|p| Some(OsStr::new("tex")) == p.extension()) {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("{DEBUG}DEBUG:{RESET} thread by path {:?}", path);
            loop {
                let modified_time = fs::metadata(&path).unwrap().modified().unwrap();
                if modified_time > hnt_time {
                    // // println!("{DEBUG}Debug:{RESET} tx: {:?}, {}", path, now());
                    tx.send(path.clone()).unwrap();
                    hnt_time = modified_time;
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    println!("{DEBUG}DEBUG:{RESET} flag");

    loop {
        let path = rx.recv().unwrap();
        compile_tex(dir);
        println!("Detected modification in {:?}, {}", path, now());
    }
    

}



use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// update .tex into .hnt by hilatex, whenever changes happen 
struct Args {
    /// directory to watch
    #[arg(default_value = ".")]
    dir: String,
}

fn main() {
    let args = Args::parse();
    watch_hnt_files(&args.dir);
}