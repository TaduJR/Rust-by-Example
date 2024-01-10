#[allow(dead_code)]
mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`")
    }

    pub fn function() {
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        pub fn function2() {
            println!("called `my_mod::nested::function()`");
        }

        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }
        
        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }
}

fn function() {
    println!("called `function()`");
}

#[allow(dead_code)]
mod my {
    pub struct OpenBox <T> {
        pub contents: T
    }

    pub struct ClosedBox <T> {
        contents: T
    }

    impl <T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}
fn main() {
    // 10 Modules
    // can be used to hierarchically split code in logical units (modules), and manage visibility 
    // (public/private) between them.

    // 10.1 Visibility
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // 10.2 Struct visibility
    let open_box = my::OpenBox { contents: "public information" };
    println!("The open box contains: {}", open_box.contents);

    let _closed_box = my::ClosedBox::new("Private information");

    // 10.3 The use declaration
    use my_mod::nested::{
        function,
        function2
    };

    use my_mod::nested::function as func;
    func();

    // 10.4 super and self
    /*  The super and self keywords can be used in the path to remove ambiguity 
        when accessing items and to prevent unnecessary hardcoding of paths. 
    */
    
}