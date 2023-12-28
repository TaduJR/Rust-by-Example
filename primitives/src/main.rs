#[allow(unused_variables)]
fn main() {
    /*
    Scalar Types
    Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    Floating point: f32, f64
    char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    bool either true or false
    The unit type (), whose only possible value is an empty tuple: ()

    Compound Types
    Arrays like [1, 2, 3]
    Tuples like (1, true)
    */

    // Variables can be type annotated.

    /*
    ** const indicates a compile-time constant which means it can only be set to a value that can be computed at compile time.
    ** let (without mut) indicates a run-time constant which means it can be initialized from the result of any valid expression but you can not re-assign it after it is first assigned.
    */
    let logical: bool = true;
    let a_float: f64 = 5.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation
    
    //Default for float: f64, integer: i32
    //2.1 Literals and Operators

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important


    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    //Operators
    //https://en.wikipedia.org/wiki/Order_of_operations#Programming_languages

    //2.2 Tuples
    // A tuple is a collection of values of different types.
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (int_param, bool_param) = pair;
        (bool_param, int_param)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);
    
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);
    
    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse(pair));

    // Tuples can be destructed
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    //Activity 1
    use std::fmt;
    #[derive(Debug)]
    struct Martix(f32, f32, f32, f32);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    //Activity 2
    fn reverse1(pair: (f32, f32)) -> (f32, f32) {
        // `let` can be used to bind the members of a tuple to variables.
        let (param1, param2) = pair;
        (param2, param1)
    }

    fn transpose(matrix: Matrix) -> Matrix {
        let (a, b): (f32, f32) = reverse1((matrix.2, matrix.0));
        let (c, d): (f32, f32) = reverse1((matrix.3, matrix.1));
        return Matrix(a, b, c, d);
    }

    let transposed: Matrix = transpose(matrix);
    println!("Transposed Matrix:\n{}", transposed);

    // 2.3 Arrays and Slices
    // An array is a collection of objects of the same type T, stored in contiguous memory.

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    //TODO Readmore on slices https://doc.rust-lang.org/rust-by-example/primitives/array.html
}