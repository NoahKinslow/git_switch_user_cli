use std::io;
use std::process::Command;

fn main() {
    let name = input_user();
    modify_git_config_user(&name);
    request_auth();
}

fn input_user() -> String {
    println!("Input desired username");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Name invalid");

    let name = name.trim();

    name.to_string()
}

fn modify_git_config_user(username: &String) {
    let change_username_command = Command::new("git")
        .args(["config", "--global", "user.name", username])
        .output()
        .expect("process to change user.name failed");

    if change_username_command.status.success() {
        println!("process to change user.name success");
    } else {
        println!("result: {}", change_username_command.status);
    }

    let change_credential_command = Command::new("git")
        .args(["config", "--global", "credential.username", username])
        .output()
        .expect("process to change credential.username failed");

    if change_credential_command.status.success() {
        println!("process to change credential.username success");
    } else {
        println!("result: {}", change_credential_command.status);
    }

    let mut username_email: String = String::new();
    username_email.push_str(username);
    username_email.push_str("@users.noreply.github.com");

    let change_email_command = Command::new("git")
        .args(["config", "--global", "user.email", &username_email])
        .output()
        .expect("process to change user.name failed");

    if change_email_command.status.success() {
        println!("process to change user.name success");
    } else {
        println!("result: {}", change_email_command.status);
    }
}

fn request_auth() {
    println!("You will now be asked to authenticate using gh");

    // execute command without capturing output of gh so that code is visible
    let _authenticate_command = Command::new("gh")
        .args(["auth", "login", "-w"])
        .spawn()
        .expect("process to authenticate failed");
}
