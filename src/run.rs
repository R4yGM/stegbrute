use std::process::Command;

pub fn extract(password: &str, file: &str, extract_file: &str, verbose: bool) -> bool{
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
        if verbose == true{
            println!("password try: {} - Failed",password);
        }
        return false
    }
}