use rpassword::prompt_password;

pub fn read_input(prompt: &str) -> String {
    let input = prompt_password(format!("Please enter the {}\n", prompt))
        .unwrap_or_else(|_| panic!("Failed to read {}", prompt));
    input.trim().to_string()
}
