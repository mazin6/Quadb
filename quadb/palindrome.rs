fn is_palindrome(s: &str) -> bool {
    let clean_string: String = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_lowercase().next().unwrap()).collect();
    let reversed_string: String = clean_string.chars().rev().collect();
    clean_string == reversed_string
}

fn main() {
    // Prompt user for input
    println!("Enter a string:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Check if input is palindrome
    let input = input.trim(); // Remove newline character
    if is_palindrome(input) {
        println!("The input string is a palindrome.");
    } else {
        println!("The input string is not a palindrome.");
    }
}
