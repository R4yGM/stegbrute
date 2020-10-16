use std::process::Command;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use std::path::Path;
use std::process;
use std::time::Instant;
use std::thread;

use std::io::{self, Write};

mod reader;

static NTHREADS: i32 = 4;
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
}
fn main() {
    separator(0);
    logo();
    let opt = Opt::from_args();
    let mut can_run: bool = false;

    if Path::new(&opt.extract_file).exists(){
        println!("\x1b[0;10;41mError:\x1b[0;0;37m The extracted file '{}' already exist \x1b[0m",opt.extract_file);
        //println!("Error : The extracted file '{}' already exist",opt.extract_file);
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
            // Spin up another thread
            children.push(thread::spawn(move || {
                let opt = Opt::from_args();
                bruteforce(&opt.wordlist,0);
                
            }));
            children.push(thread::spawn(move || {
                let opt = Opt::from_args();
                bruteforce(&opt.wordlist,1);
            }));
            children.push(thread::spawn(move || {
                let opt = Opt::from_args();
                bruteforce(&opt.wordlist,2);

            }));
            children.push(thread::spawn(move || {
                let opt = Opt::from_args();
                bruteforce(&opt.wordlist,3);

            }));
            children.push(thread::spawn(move || {
                let opt = Opt::from_args();
                bruteforce(&opt.wordlist,4);

            }));
            children.push(thread::spawn(move || {
                let opt = Opt::from_args();
                bruteforce(&opt.wordlist,5);

            }));
    
        for child in children {
            // Wait for the thread to finish. Returns a result.
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
    let mut init_count = 0;
    let mut old_count = count;

    while let Some(line) = reader.read_line(&mut buffer) {
        if init_count > count{
        if count == old_count + 6 || count <= 5{
        old_count = count;
        let res = extract(line?.trim(), &opt.file_name, &opt.extract_file);
        if res{
            println!("Successfully cracked in {:.2?}",before.elapsed());
            separator(0);
            process::exit(0x0100);
            }
        }
    }
        init_count += 1;
        count += 1;
    }
    println!("Failed to crack the file, finished the passwords {:.2?}",before.elapsed());
    Ok(false)
}
fn extract(password: &str, file: &str, extract_file: &str) -> bool{
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
        println!("password try: {} - Failed",password);
        return false
    }
}
pub fn separator(mut count: usize){
    if count <= 0{
         count = 60;
    }
    print!("\x1b[0;34m{:=<1$}\x1b[0m\n","",count);
}
pub fn logo(){
let logo = r#"     ____  _             ____             _       
    / ___|| |_ ___  __ _| __ ) _ __ _   _| |_ ___ 
    \___ \| __/ _ \/ _` |  _ \| '__| | | | __/ _ \
     ___) | ||  __/ (_| | |_) | |  | |_| | ||  __/
    |____/ \__\___|\__, |____/|_|   \__,_|\__\___|
                   |___/                          
"#;
println!("{}\nStegBrute v{} - By R4yan\nhttps://github.com/R4yGM/StegBrute\n",logo,VERSION);
}
