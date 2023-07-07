use am::{simple_func_like_macro, JustForFunNoDerive};

fn main() {
    println!("Hello, Macro-World!");

    // try to invoke the function which simple_func_like_macro declared
    let lucky_num = give_me_the_lucky_number();

    println!("The number from func like macro: {}", lucky_num);

    let _ = FirstUnitStruct;
}

// let the macro create a function definition
simple_func_like_macro!(everything, in, here, will, be, ignored);

// derive macro
#[derive(JustForFunNoDerive)]
struct FirstUnitStruct;
