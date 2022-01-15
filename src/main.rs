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
    execute_git_config_command(&String::from("user.name"), username);

    execute_git_config_command(&String::from("credential.username"), username);

    let (user_id, username) = request_get_user_id(username);

    let username_email = format!("{}+{}@users.noreply.github.com", user_id, username);

    execute_git_config_command(&String::from("user.email"), &username_email);
}

fn execute_git_config_command(field_name: &String, field: &String) {
    let change_field_command = Command::new("git")
        .args(["config", "--global", &field_name, &field])
        .output()
        .expect(format!("process to change {} failed", &field_name).as_str());

    if change_field_command.status.success() {
        println!("process to change {} success", &field_name);
    } else {
        println!("result: {}", change_field_command.status);
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

// return (username, user_id)
fn request_get_user_id(username: &String) -> (String, String) {
    let temp_username = String::from(username);

    let api_path = format!("https://api.github.com/users/{}", username);

    println!("api path: {}", api_path);

    let change_email_command = Command::new("gh")
        //.args(["api", api_path.as_str(), "--jq", "'.id'"])
        .args(["api", api_path.as_str(), "--jq", ".id"])
        .output()
        .expect("process to fetch email from git api failed");

    let user_id = String::from(
        String::from_utf8(change_email_command.stdout)
            .unwrap()
            .trim(),
    );

    println!("user_id: {}", user_id);

    (temp_username, user_id)
}
