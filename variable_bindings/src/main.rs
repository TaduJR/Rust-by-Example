fn main() {
    // 4. Variable Bindings
    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    //4.1 Mutability
    let mut _mutable_binding = 1;

    //4.2 Scope and Shadowing
    /* 
        variable shadowing occurs when a variable declared within a certain scope 
        (decision block, method, or inner class) has the same name as a variable declared in an outer scope.
    */

    //4.3 Declare first

    //4.4 Freezing
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `_mutable_integer` goes out of scope
    }
}