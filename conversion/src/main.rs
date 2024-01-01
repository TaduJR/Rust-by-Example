use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

/*
impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}
*/

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}
fn main() {
    //TODO Readmore about From and Into
    //From and Into
    //From
    //The From trait allows for a type to define how to create itself from another type
    let num = Number::from(30);
    println!("My number is {:?}", num);

    //Into
    /*
    The Into trait is simply the reciprocal of the From trait. 
    That is, if you have implemented the From trait for your type, Into will call it when necessary.
    */

    //Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.


    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
}