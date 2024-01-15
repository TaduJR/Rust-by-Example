struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}
fn main() {
    // 15. Scoping Rules
    // 15.1 RAII (Resource Acquisition Is Initialization)
    let _x = ToDrop;
    println!("Made a ToDrop!");

    // 15.2 Ownership and moves
    /*
        Because variables are in charge of freeing their own resources, resources can only have one owner.
        This prevents resources from being freed more than once.
    */

    // 15.2.2 Partial Moves
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

    // 15.3 Borrowing

    // 115.3.1 Mutability
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    fn borrow_book(book: &Book) {
        println!(
            "I immutably borrowed {} - {} edition",
            book.title, book.year
        );
    }

    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    // 15.3.2 Aliasing
    // NOTE: https://doc.rust-lang.org/rust-by-example/scope/borrow/alias.html

    // 15.3.3 The ref pattern

    // 15.4 Lifetimes

    // 15.4.1 Explicit annotation
}
