#![allow(unused_variables)]
#[cfg(target_os = "windows")]
fn are_you_on_windows() {
    println!("You are running windows!");
}
fn main() {
    // 13. Attributes
    /* Attributes look like #[outer_attribute] or #![inner_attribute], with the difference between 
       them being where they apply.

       ** #[outer_attribute] applies to the item immediately following it.
       ** 
    */

    // 13.1 Dead Code

    // 13.2 Crates

    // 13.3 cfg
    are_you_on_windows();
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
