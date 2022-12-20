
fn password_analyze(password: &str) -> u32 {
    let mut score = 0;

    // Check for length
    if password.len() >= 8 {
        score += 5;  // Passwords greater than 8 characters in length are worth 5 points
    }

    // Check for length
    if password.len() >= 5 {
        score += 3;   // Passwords greater than 5 characters in length are worth 3 points
    }

    // Check for uppercase letters
    if password.chars().any(|c| c.is_uppercase()) {
        score += 4;  // Each uppercase letter in the password is worth 4 points
    }

    // Check for lowercase letters
    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;  // Each lowercase letter in the password is worth 1 point
    }

    // Check for numbers
    if password.chars().any(|c| c.is_numeric()) {
        score += 2;  // Each number in the password is worth 2 points
    }

    // Check for special characters
    if password.chars().any(|c| !c.is_alphanumeric()) {
        score += 5;  // Each special character in the password is worth 5 points
    }

    score
}

fn main() {
    let password = "password";
    let score = password_analyze(password);
    println!("The password '{}' has a strength score of {}", password, score);
}