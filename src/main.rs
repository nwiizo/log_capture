use std::process::{Child, Command, Stdio};
use std::thread;



fn operation(srcdir: &str,user: &str,host: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("{}{}{}{}{}{}", "rsync -av -e ssh --existing --include='*.gz'", srcdir,"/",user,"@",host))
        .output()
        .expect("failed to execute process");
    let operation = output.stdout;
    println!("{}", std::str::from_utf8(&operation).unwrap());
}

fn main() {

    for line in 1..11 {
        println!("{} !!!!!!!!!",line);
        operation("/var/log","motouchi","127.0.0.1");
    }

}
