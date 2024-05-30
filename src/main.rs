const MAIN : &'static str = "main";
const TEX_MAIN : &'static str = "./main.tex";
const HNT_MAIN : &'static str = "./main.hnt";
const BIB_MAIN : Option<&'static str> = Some("./ref.bib");

use std::path::PathBuf;
use std::time::{SystemTime, Duration};
use std::sync::mpsc;
use std::thread;
use std::process::Command;

use inline_colorization::{
    color_red as ERROR,
    // color_magenta as DEBUG, 
    color_bright_blue as INFO, 
    color_yellow as WARN, 
};
const RESET: &'static str = "\u{1b}[39m\u{1b}[49m";

use chrono::Local;
fn now() -> String {Local::now().format("%H:%M:%S").to_string()}

use std::io::{self, Write, BufRead};
fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}

use std::str::from_utf8;
fn compile_tex() -> bool {
    let output = Command::new("hilatex")
                                        .arg(TEX_MAIN)
                                        .output().expect("failed to execute process");
    let e = from_utf8(&output.stdout).unwrap();
    // println!("{DEBUG}Debug:{RESET} Output: {}", e);

    let mut flag = true; // means no error
    for error_line in e.lines()
                        .filter(|line| line.starts_with("!")) {
        println!("{ERROR}{}{RESET} -- hilatex", error_line);
        flag = false;
    }

    flag
}
fn compile_bib() -> bool {
    let output = Command::new("biber")
                                        .arg(MAIN)
                                        .output().expect("failed to execute process");
    let e = from_utf8(&output.stdout).unwrap();
    // println!("{DEBUG}Debug:{RESET} Output: {}", e);

    let mut flag = true; // means no error
    for error_line in e.lines() {
        if error_line.starts_with("ERROR") {
            println!("{ERROR}{}{RESET} -- biber", error_line);
            flag = false;
        } else if error_line.starts_with("WARN") {
            println!("{WARN}{}{RESET}", error_line);
        }
    }

    flag
}
fn compile_init() {
    compile_tex();
    if BIB_MAIN.is_some() {
        compile_bib();
        compile_tex();
    }
}

#[derive(Debug)]
enum FileType {
    TEX,
    BIB
}

use std::fs::metadata;
fn send_by_changes(tx: &mpsc::Sender<(FileType, PathBuf)>, path: &PathBuf, hnt_time: SystemTime, file_type: FileType) -> SystemTime {
    let modified_time = metadata(path).unwrap().modified().unwrap();
    if modified_time > hnt_time {
        // println!("{DEBUG}Debug:{RESET} tx: {:?}, {}", path, now());
        tx.send((file_type, path.clone())).unwrap();
        modified_time
    } else {
        hnt_time
    }
}

use walkdir::WalkDir;
use std::ffi::OsStr;
fn watch_hnt_files() {
    let (tx, rx) = mpsc::channel();
    let mut hnt_time = SystemTime::now();

    let dir_entries = WalkDir::new(".");

    for path in dir_entries.into_iter().filter_map(|e| e.ok())
                                    .map(|e| e.into_path())
                                    .filter(|p| Some(OsStr::new("tex")) == p.extension()) {
        let tx = tx.clone();
        println!("{INFO}INFO:{RESET} thread by path {:?}", path);
        thread::spawn(move || {
            loop {
                hnt_time = send_by_changes(&tx, &path, hnt_time, FileType::TEX);
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    // watch the ref.bib
    if let Some(path) = BIB_MAIN {
        let tx = tx.clone();
        let path = PathBuf::from(path);
        println!("{INFO}INFO:{RESET} thread by path {:?}", path);
        thread::spawn(move || {
            loop {
                hnt_time = send_by_changes(&tx, &path, hnt_time, FileType::BIB);
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    // spawn a new thread if a new `.tex` created
    let tx = tx.clone();
    thread::spawn(move || {
        loop {
            // not perfect, but works
            let path = PathBuf::from(input("> ").unwrap()).with_extension("tex"); 

            let output = Command::new("touch").arg(path.to_str().unwrap()).output().expect("failed to execute process");
            println!("{}", from_utf8(&output.stdout).unwrap());
        
            if path.is_file() {
                let tx = tx.clone();
                println!("{INFO}INFO:{RESET} thread by path {:?}", path);
                thread::spawn(move || {
                    loop {
                        hnt_time = send_by_changes(&tx, &path, hnt_time, FileType::TEX);
                        thread::sleep(Duration::from_millis(100));
                    }
                });
            } else {
                println!("{WARN}WARN:{RESET} No such a .tex file");
            }
        }
    });

    loop {
        let (file_type, path) = rx.recv().unwrap();
        println!("{INFO}INFO:{RESET} Detected modification in {:?}, {}", path, now());
        match file_type {
            FileType::TEX => {
                compile_tex();
            }
            FileType::BIB => {
                if compile_bib() == false {
                    compile_tex();
                    if compile_bib() == false {
                        panic!("{ERROR}I think you need to fix it manually{RESET}")
                    }
                }
                compile_tex();
                compile_bib();
            }
        }
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

fn check_consts() {
    if let Some(bib_main_path) = BIB_MAIN {
        if ! PathBuf::from(bib_main_path).is_file() {
            panic!("{ERROR}no such a file:{RESET} {}", bib_main_path)
        }

    }
    if ! PathBuf::from(TEX_MAIN).is_file() {
        panic!("{ERROR}no such a file:{RESET} {}", TEX_MAIN)
    }
}

use std::env::set_current_dir;
fn main() {
    let args = Args::parse();
    if let Err(e) = set_current_dir(&args.dir) {
        panic!("{:?}", e);
    } else {
        check_consts();
        println!("{INFO}INFO:{RESET} initialising",);
        compile_init();
        if cfg!(target_os = "macos") {
            let _ = Command::new("open").arg(HNT_MAIN).output().expect("failed to execute process");
        }
        println!("{INFO}INFO:{RESET} {HNT_MAIN} opened");
        watch_hnt_files();
    }
}