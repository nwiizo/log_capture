use std::process::{Child, Command, Stdio};
use std::thread;


//rsync -av -e ssh motouchi@127.0.0.1:/tmp/test.txt /tmp/samples/
fn operation(srcdir: &str,dstdir: &str,user: &str,host: &str) {
    let output = Command::new("rsync ")
        .arg("-av -e ssh ")
        .arg(format!("{}{}{}:{} {}",user,"@",host,srcdir,dstdir))
        .output()
        .expect("failed to execute process");
    let operation = output.stdout;
    println!("{}", std::str::from_utf8(&operation).unwrap());
}

fn main() {

    for line in 1..11 {
        println!("{} !!!!!!!!!",line);
        operation("/tmp/test.txt","/tmp/samples/","motouchi","127.0.0.1");
    }

}
