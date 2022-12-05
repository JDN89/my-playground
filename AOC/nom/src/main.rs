fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(
            cleanup_part_two("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string()),
            4
        );
    }
}
