use tokio::process::Command;

#[tokio::main]
async fn main() {
    let command_ls = "ls";
    let command_pwd = "pwd";


    let output_ls = Command::new(command_ls).args(&["-l", "-a"]).output().await.expect("Problem in ls");
    let output_pwd = Command::new(command_pwd).output().await.expect("Problem in pwd");

    println!("Output of '{}':\n{}", command_ls, String::from_utf8_lossy(&output_ls.stdout));
    println!("Output of '{}':\n{}", command_pwd, String::from_utf8_lossy(&output_pwd.stdout));
}