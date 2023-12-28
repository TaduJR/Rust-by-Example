#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    //3. Custom Types
    //3.1 Structures
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
}