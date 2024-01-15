use std::fmt::{Display, Debug};

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // 15. Generis
    // Generics is the topic of generalizing types and functionalities to broader cases.

    //14.1 Functions

    //14.2 Implementation
    struct Val { val: f64 }
    struct GenVal<T> { gen_val: T }

    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    impl<T> GenVal<T> {
        fn value(&self) ->&T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());

    struct Empty;
    struct Null;
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an
        // additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;

    // 14.4 Bounds
    /* 
        When working with generics, the type parameters often must use traits as bounds to stipulate 
        what functionality a type implements. 
    */
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }

    struct S<T: Display>(T);
    //let s = S(vec![1]);

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);

    //14.6 Where clauses

    //14.7 New Type Idiom
    /* The newtype idiom gives compile time guarantees that the right type of value is supplied to a program. */

    //TODO Read more: Associated items and Phantom type parameters

    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
