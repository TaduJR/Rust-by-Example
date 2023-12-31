#[allow(unused_variables)]
fn main() {
    // 5.1 Casting
    // explicit type conversion (casting) can be performed using the as keyword.
    let decimal = 62.4_f32;
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    //let character = decimal as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    //TODO https://doc.rust-lang.org/rust-by-example/types/cast.html

    // 5.2 Literals
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));

    // 5.3 Interence
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    // 5.4 Aliasing
    type NanoSecond = u64;
    let nanoseconds: NanoSecond = 5 as u64;
}