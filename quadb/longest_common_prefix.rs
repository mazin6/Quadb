fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let shortest = strings.iter().map(|s| s.len()).min().unwrap();
    let mut prefix = String::new();

    for i in 0..shortest {
        let first_char = strings[0].chars().nth(i).unwrap();
        if strings.iter().all(|s| s.chars().nth(i).unwrap() == first_char) {
            prefix.push(first_char);
        } else {
            break;
        }
    }

    prefix
}

fn main() {
    let strings1 = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let strings2 = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];

    println!("Longest common prefix of strings1: {}", longest_common_prefix(&strings1)); // Output: "fl"
    println!("Longest common prefix of strings2: {}", longest_common_prefix(&strings2)); // Output: ""
}
