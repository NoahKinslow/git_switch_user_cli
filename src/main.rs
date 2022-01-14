use std::io;

fn main() {
    let name = input_user();
    modify_git_config_user(name);
}

fn input_user() -> String {
    println!("Input desired username");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Name invalid");

    let name = name.trim();

    name.to_string()
}

fn modify_git_config_user(username: String) {
    
}
