use am::{simple_func_like_macro, simple_proc_macro_attribute, JustForFunNoDerive};

fn main() {
    println!("Hello, Macro-World!");

    // try to invoke the function which simple_func_like_macro declared
    let lucky_num = give_me_the_lucky_number();

    println!("The number from func like macro: {}", lucky_num);

    let _ = FirstUnitStruct {
        field: "string".to_owned(),
        field_two: "string".to_owned(),
    };
}

// let the macro create a function definition
simple_func_like_macro!(everything, in, here, will, be, ignored);

// derive macro
#[derive(JustForFunNoDerive)]
pub struct FirstUnitStruct {
    #[field_one]
    pub field: String,
    #[field_two]
    pub field_two: String,
}

#[simple_proc_macro_attribute]
pub fn i_will_annotated(arg: String) {
    println!("Hello from method - {}", arg);
}
