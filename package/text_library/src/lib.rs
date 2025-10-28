pub mod maths;

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn is_palindrome(input: &str) -> bool {
    let cleaned_input: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    cleaned_input == reverse(&cleaned_input)
}

pub fn demo_add() {
    let result = maths::add(5, 8);
    println!("5 + 8 = {}", result);
}

pub fn demo_sub() {
    let result = maths::sub(9, 2);
    println!("Result = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_reverse() {
        assert_eq!(reverse("wizard"), "draziw");
    }

    #[test]
    fn text_is_palindrome() {
        assert!(is_palindrome("A man, a plan, a canal, Panama"));
    }

    #[test]
    fn test_add() {
        assert_eq!(maths::add(2, 3), 5);
    }

    #[test]
    fn test_sub() {
        assert_eq!(maths::sub(8, 3), 5);
    }
}
