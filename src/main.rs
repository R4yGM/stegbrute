use std::process::Command;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use std::path::Path;
use std::process;
use std::time::Instant;
use std::thread;
use std::fs::File;
use std::io::{self, Write, prelude::*, BufReader};

mod logging;
mod reader;

//static NTHREADS: i32 = 3;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, StructOpt)]
#[structopt(name = "StegBrute",about = "Steganography bruteforce tool",version = VERSION, author = "R4yan <yessou.rayan@gmail.com>")]
struct Opt {
    #[structopt(short = "x", long = "extract-file",default_value ="stegbrute_results.txt", help = "the file name path where you want to write the results")]
    extract_file: String,
    #[structopt(short = "w", long = "wordlist", required=true, help = "path of the wordlist")]
    wordlist: String,
    #[structopt(short = "f", long = "file-name", required=true, help = "the file name path you want to crack")]
    file_name: String,
    #[structopt(short = "v", long = "verbose", help = "shows every try the program does")]
    verbose: bool,
    #[structopt(short = "t", long = "threads",default_value = "3", help = "number of threads to bruteforce the file")]
    threads: i32,
}
fn main() {
    logging::separator(0);
    logging::logo();
    let opt = Opt::from_args();
    let mut can_run: bool = false;
    if Path::new(&opt.extract_file).exists(){
        println!("\x1b[0;10;41mError:\x1b[0;0;37m The extracted file '{}' already exist \x1b[0m",opt.extract_file);
        return;
    }

    if Path::new(&opt.wordlist).exists() == false{
        println!("Error :the wordlist '{}' doesn't exist",opt.wordlist);
        return;
    }else{can_run = true}
    if Path::new(&opt.file_name).exists() == false{
        println!("the file '{}' doesn't exist",opt.file_name);
        return;
    }else{can_run = true}
    if can_run{ 
        let mut children = vec![];
        let mut stop: bool = false;
        if opt.verbose == false {
            println!("Bruteforcing the file '{}' with the wordlist '{}' using {} threads",opt.file_name,opt.wordlist,opt.threads);
        }
        for i in 0..opt.threads {
            children.push(thread::spawn(move || {
                let opt = Opt::from_args();
                bruteforce(&opt.wordlist,i); 
            }));
        }
        for child in children {
            let _ = child.join();
        }
    }   
    return;
}

fn bruteforce(file_name: &str, mut count: i32) -> io::Result<bool> {
    let before = Instant::now();
    let opt = Opt::from_args();

    let mut reader = reader::BufReader::open(file_name)?;
    let mut buffer = String::new();
    let thread_n = count;
    let mut init_count = 0;
    let mut old_count = count;
    while let Some(line) = reader.read_line(&mut buffer) {
        if count == old_count + &opt.threads+1 || init_count == old_count{
            old_count = count;
            let res = extract(line?.trim(), &opt.file_name, &opt.extract_file);
            if res{
                println!("\x1b[1;1mTried passwords : {}",count);
                println!("Successfully cracked in {:.2?}\x1b[0m",before.elapsed());
                logging::separator(0);
                process::exit(0x0100);
            }
        }
        init_count += 1;
        count += 1;
    }
    println!("(thread-{}) Failed to crack the file, finished the passwords {:.2?}",thread_n,before.elapsed());
    Ok(false)
}
fn extract(password: &str, file: &str, extract_file: &str) -> bool{
    let opt = Opt::from_args();
    let output = Command::new("steghide")
                     .arg("extract")
                     .arg("-sf")
                     .arg(file)
                     .arg("-xf")
                     .arg(extract_file)
                     .arg("-p")
                     .arg(password)
                     .arg("-f")
                     .output()
                     .expect("failed to execute process");
    if String::from_utf8_lossy(&output.stderr).contains("wrote extracted data to"){
        println!("\x1b[1;32mpassword try: {0} - Success \nFile extracted!\x1b[0m\n\x1b[1;1mPassword: {0}\nResults written in: {1}\x1b[0m",password,extract_file);
        return true
    }else{
        if opt.verbose == true{
            println!("password try: {} - Failed",password);
        }
        return false
    }
}
