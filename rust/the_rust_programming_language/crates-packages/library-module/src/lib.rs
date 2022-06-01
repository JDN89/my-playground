// A binary crate should generate an executable (or multiple) that can be installed in the user's path
// and can be executed as usual. The purpose of a library crate on the other hand
//is not to create executables but rather provide functionality for other crates to depend on and use.
//Show activity on this post.

//To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.

//A path can take two forms:

//  An absolute path starts from a crate root by using a crate name or a literal crate.
//A relative path starts from the current module and uses self, super, or an identifier in the current module.

// Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

// Moduels zijn ook handig om de grenzen van je code af te bakenen
// Moduels zijn private by default
// child modules hebben toegang tot parent moduels, maar parent moduels hebben by default geen toegang to private child moduels
// reden -> child moduels hebben kennis van de context waaring ze gedefineerd zijn

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                //The super keyword is used to access the grandparent module from the current module.
                //It enables us to access the private functions of the parent module.
                super::serve_order();
            }
            fn cook_order() {}
            fn take_payment() {}
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
