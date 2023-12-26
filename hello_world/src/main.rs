fn main() {
    //This is one line comment

    /*
    This is a 
    multi line comment
    */

    println!("Hello, world!");

    /* 1.2 Formatting */
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}", 
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    //Special formatting
    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420);
    println!("Base 8: {:o}", 69420);
    println!("Base 16: {:x}", 69420);

    //Padding
    println!("{number:>5}", number=1); //    1
    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000
    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0<width$}", number=1, width=6); // 100000
    println!("{number:>width$}", number=1, width=6); //

    //fmt:Display
    // Only types that implement fmt::Display can be formatted with `{}`.
    // User-defined types do not implement fmt::Display by default.
    #[allow(dead_code)]
    struct StructureNoWarn(i32);
    // println!("This struct `{}` won't print...", Structure(3)); //error[E0277]: `Structure` doesn't implement `std::fmt::Display`

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    
    //Excerisce
    let pi: f64 = 3.141592;
    println!("Pi is roughly {pi:.3}");

    // https://doc.rust-lang.org/std/fmt/
    // std::fmt
    // Formatting Parameters
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    /* This is a parameter for the “minimum width” that the format should take up. 
        If the value’s string does not fill up this many characters, then the padding specified by fill/alignment 
        will be used to take up the required space (see below).
    */

    /*
    The value for the width can also be provided as a usize in the list of parameters 
    by adding a postfix $, indicating that the second argument is a usize specifying the width. 
     */
    println!("Hello {:width$}", "x", width = 5);
    let width = 5;
    println!("Hello {:width$}", "x");

    //1.2.1 Debug
    #[derive(Debug)]
    struct DebugPrintable(i32);
    println!("Now {:?} will print!", DebugPrintable(7));

    //1.2.2 Display
    use std::fmt;
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicated whether the operation succeeded or failed.
            write!(f, "{}", self.0)
        }
    }

    //Activity
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.x, self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    //Testcase: List
    struct List(Vec<i32>);

    //Activity
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count == vec.len() - 1 { 
                    write!(f, "{}: {}", count, v)?; 
                    break;
                }
                write!(f, "{}: {}, ", count, v)?;
                /*
                // [1, 2, 3] Formatter
                if count != 0 { write!(f, ",")?; }
                write!(f, "{}, ", v)?;
                */
            }
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
