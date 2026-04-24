use std::{io::stdin, process::Stdio};

use tokio::{io::AsyncWriteExt, process::Command};
use shell_words;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt};


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
    // trial_2_using_piped().await;
    trial_3_using_env_vars().await;
}

async fn trial_1(){
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

async fn trial_2_using_piped(){
    println!("current dir: {:?}", std::env::current_dir().unwrap());
    let mut child = Command::new("./data_reader.sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    println!("Child process spawned with PID: {}", child.id().unwrap());

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    println!("Child process stdin and stdout pipes created");

    // Write to the child's stdin
    stdin.write("Hi from rust".as_bytes()).await.expect("Failed to write to stdin");
    drop(stdin);

    println!("Data written to child process stdin");

    // Read from the child's stdout
    let mut stdout = child.stdout.take().expect("Failed to open stdout");
    let mut response_buffer = String::new();
    
    // Read the entire output into a string
    stdout.read_to_string(&mut response_buffer).await.expect("Failed to read from stdout");

    // 3. Wait for exit
    let _ = child.wait().await.expect("Failed to wait for child process");

    println!("Child process output: {}", response_buffer);
}


async fn trial_3_using_env_vars(){

    let output = Command::new("./data_reader_env_var.sh")
        .env("VAR_DRASI", "Hello from Rust env var")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let mut stdout = output.stdout.expect("Failed to capture stdout");
    let mut response_buffer = vec![0u8; 1024];
    let n = stdout.read(&mut response_buffer).await.expect("Failed to read from stdout");

    println!("Child process output: {}", String::from_utf8_lossy(&response_buffer[..n]));
}