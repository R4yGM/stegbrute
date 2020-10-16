const VERSION: &'static str = env!("CARGO_PKG_VERSION");
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