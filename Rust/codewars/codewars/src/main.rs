fn solution(word: &str, ending: &str) -> bool {
    if ending.len() > word.len() { return false} 

    let len = word.len() - ending.len();
    let slice = &word[len..];
    println!("{}",slice);
    if slice == ending {
        return true;
    }
    false
}

fn main() {
    let result = solution("abd", "abd");
    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(true, solution("abc", "abc"));
    }
    #[test]
    fn returns_false() {
        assert_eq!(false, solution("strawberry", "banana"));
    }
}
