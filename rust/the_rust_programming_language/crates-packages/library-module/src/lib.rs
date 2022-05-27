// A binary crate should generate an executable (or multiple) that can be installed in the user's path
// and can be executed as usual. The purpose of a library crate on the other hand
//is not to create executables but rather provide functionality for other crates to depend on and use.
//Show activity on this post.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
