use std::option;

#[allow(unused_variables)]
#[allow(unused_labels)]
#[allow(unreachable_code)]
#[allow(dead_code)]
fn main() {
    //8. Flow of Control
    //8.1 If/Else
    
    //8.2 Loop
    // Rust provides a loop keyword to indicate an infinite loop.

    //8.2.1 Nesting and label
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    //8.2.2 Returning from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };   
    println!("Result of the above loop: {}", result);

    //8.4 for loops
    //for and range
    for n in 1..10 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }

    //for and iterators
    

    {
        /* 
        iter - This borrows each element of the collection through each iteration. 
        Thus leaving the collection untouched and available for reuse after the loop.
        */
        let names = vec!["Bob", "Frank", "Ferries"];
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        println!("names: {:?}", names);
    }

    {
        /*
        into_iter - This consumes the collection so that on each iteration the exact data is provided. 
        Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
        */

        let names = vec!["Bod", "Frank", "Ferris"];
        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        // println!("names: {:?}", names);
    }

    {
        /*
        iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
        */

        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
    }

    // 8.5 match
    /* 
        Rust provides pattern matching via the match keyword, which can be used like a C switch. 
        The first matching arm is evaluated and all possible values must be covered.
    */

    let number = 13;
    println!("Tell me about {}", number);

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    // 8.5.1 Destructuring
    // 8.5.1.1 tuples
    let triple = (0, -2, -3);
    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    //8.5.1.2 arrays/slices
    let array = [1, -2, 6, 7];

    match array {
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    //8.5.1.3 Enums
    #[allow(dead_code)]
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }

    //8.5.1.4 pointers/ref
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y} => println!("First"),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };
    let Foo {x: x0, y: y0} = faa;
    println!("{:?} {}", x0, y0);

    //8.5.2 Guards
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celsius(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }

    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
    }

    // 8.5.3 Binding
    // TODO Read More about binding in rust
    fn age() -> u32 {
        15
    }

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }

    // 8.6 if let
    let optional = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = optional {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    // 8.7 let-else
    use std::str::FromStr;
    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item)
    }
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // 8.8 while-let
    let mut optional1 = Some(0);
    while let Some(i) = optional1 {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional1 = None;
        } else {
            println!("Greater than 9, quit!");
            optional1 = Some(i + 1);
        }
    }

}