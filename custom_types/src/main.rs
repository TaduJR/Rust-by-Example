enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

#[allow(dead_code)]
enum Operations {
    Add,
    Subtract
}
#[allow(dead_code)]
enum Status {
    Poor,
    Rich
}

#[allow(dead_code)]
enum Work {
    Civilian,
    Solider
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
fn main() {
    //3. Custom Types
    //3.1 Structures
    /* Types of Structures
        Tuple structs, which are, basically, named tuples.
        The classic C structs
        Unit structs, which are field-less, are useful for generics.
     */
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    struct Unit; //Unit Struct
    struct Pair(f32, f32); //Tuple Struct
    struct Point {
        x: f32,
        y: f32
    }
    struct Rectangle {
        top_left: Point,
        bottom_right: Point
    }

    //Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    let bottom_right = Point { x: 5.2, ..point};
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1.0, 0.1);

    //Activity 1
    fn rect_area(rect: Rectangle) -> f32 {
        let Rectangle { top_left: Point {x: x1, y: y1} ,  bottom_right: Point {x: x2, y: y2} } = rect;
        let width: f32 = x2 - x1;
        let height: f32 = y2 - y1;
        return (width * height).abs();
    }
    println!("Area of rectangle: {}", rect_area(_rectangle));


    // 3.2 Emuns
    // The enum keyword allows the creation of a type which may be one of a few different variants.
    /* Types of Enums 
        c-like structures
        like tuple structs
        unit-like
    */
    
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("Page Loaded"),
            WebEvent::PageUnload => println!("Page Unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            },
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    print!("\n");
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    //Type Alias
    type WebE = WebEvent;
    
    impl Operations {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
    
    let add = Operations::Add;
    println!("{}", add.run(5, 5));

    // 3.2.1. use
    use crate::Status::{Poor, Rich};
    use crate::Work::*;
    
    //3.2.2 c-like enum

    //3.2.3 Testcase: linked-list
    //TODO https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html

    // 3.3 Constants
    // Types of Constants in Rust
    // 1. Static    2. Const
}