use std::{io::stdin, process::Stdio};

use tokio::process::Command;
use shell_words;
use tokio::io::{self, AsyncBufReadExt};

// #[tokio::main]
// async fn main() {
//     let command_ls = "ls";
//     let command_pwd = "pwd";


//     let output_ls = Command::new(command_ls).args(&["-l", "-a"]).output().await.expect("Problem in ls");
//     let output_pwd = Command::new(command_pwd).output().await.expect("Problem in pwd");

//     println!("Output of '{}':\n{}", command_ls, String::from_utf8_lossy(&output_ls.stdout));
//     println!("Output of '{}':\n{}", command_pwd, String::from_utf8_lossy(&output_pwd.stdout));
// }

#[tokio::main]
async fn main() {
    let argv = &["bash"];

    println!("Executing: {}", shell_words::join(argv));

    let shell_joined = shell_words::join(argv);

    println!("Shell splitted command: {}", shell_words::split(&shell_joined).unwrap().join(", "));

    let timeout = std::time::Duration::from_secs(1);

    let mut pid = tokio::process::Command::new(&argv[0])
        .kill_on_drop(true)
        .args(&argv[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to start");


    match tokio::time::timeout(timeout, pid.wait()).await {
        Ok(Ok(status)) => println!("Process exited with status: {}", status),
        Ok(Err(e)) => eprintln!("Error waiting for process: {}", e),
        Err(_) => eprintln!("Process timed out and was killed"),
    }
    

    println!("Process exited with: {:?}", pid);




    let mut input = String::new();
    let mut reader = io::BufReader::new(io::stdin());

    reader.read_line(&mut input).await.unwrap();

    println!("You typed: {:?}", input);
}