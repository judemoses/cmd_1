use std::process::Command;
use std::io;
use std::io::Write;

fn main(){


    let cmd=Command::new("ls")
                            .arg("-l")
                            .arg("-a")
                            .output()
                            .expect("ls command failed to start");
    io::stdout().write_all(&cmd.stdout).unwrap();


}
