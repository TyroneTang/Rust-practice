const GLOBAL_VAR_STR: &str = &"abc";


fn primitive_data_types(is_enabled: bool) {
    if !is_enabled {
        println!("Primitive data types are disabled. Skipping func...");
        return;
    }

    let a: i32 = -10;
    let ua: u32 = 100;
    let b: f64 = 3.142;
    let c: bool = true;
    let d: char = 'A'; // Note: char literals are single quotes in Rust
    println!("Primitive data types are \n{a}, \n{ua}, \n{b}, \n{c}, and \n{d}.");
}

fn compound_data_types(is_enabled: bool) {
    if !is_enabled {
        println!("Compound data types are disabled. Skipping func...");
        return;
    }

    // includes tuples and arrays.
    let nums: [i32; 5] = [1,2,3,4,5];

    // &str lives in binary and points to it
    // String::from("ssadsa") allocates heap memory to it. + COPY. Dynamic
    let tup: (i32, f64, &str, bool) = (1, 3.142, "hello", true);
    println!("Compound data types are \n{nums:?} and \n{tup:?}.");
    println!("Accessing indivdual nums el::: \n{}, \n{}, \n{}", nums[0], nums[1], nums[2]);
    println!("Accessing individual enum el::: \n{}, \n{}, \n{}", tup.0, tup.1, tup.2);

    // stack-allocated integer
    let al_int: i32 = 10;
    // stack-allocated referenced(pointer) to an integer in binary
    // this is more inefficient compared to stack-allocated integer, ref is 8bytes, stack allocated is 4bytes. 
    let ref_int: &i32 = &20;

    println!("Stack allocated int {al_int}");
    println!("stack allocated ref pointer int {ref_int}");

    // heap-allocated integer (dynamic) & copied
    let al_str: String = String::from("hello!!!");
    // stack-allocated referenced(pointer) to an string in binary
    let ref_str: &str = &"abc";

    println!("Heap allocated dynamic string {al_str}");
    println!("stack allocated ref pointer string {ref_str}");


    // mutation, needs to be owned.
    // stack = no mutable allowed datatypes. only heap
    let mut base_str: String = String::from("String is going to be ...");
    base_str.push_str("mutated");
    println!("{base_str}");

    // sliced & refrerencing
    let new_str: String = String::from("new string to be referenced!");
    let ref_new_str: &str = &new_str;
    let str_slicing: &str = &new_str[0..8];
    println!("{ref_new_str} and slicing {str_slicing}");


    return;
}


fn rust_function(is_enabled: bool) {
    if !is_enabled {
        println!("disabled rust_function skipping...");
        return;
    }

    // not idomatic, move outside to module level
    // nested function is a code smell
    fn check_height(height: u32) {
        println!("check height {height}");
    }
    check_height(182);

    // use this instead
    let get_height = |height:f64| println!("check new height {height}");
    get_height(188.0);

    let get_biodata = |name: &str, age: u32, height: f32| {
        println!("check name {name}, age {age}, and height: {height}cm");
    };
    get_biodata("andy", 15, 177.0);


    // fn human_id(name: &str, age: u32, height: f32) {
    //     println!("check name {name}, age {age}, and height: {height}cm");
    // }
    // human_id("andy", 15, 177.0);
    
    println!("checking globar var {GLOBAL_VAR_STR}");

    let result:i32 = {
        let price: i32 = 10;
        let qty: i32 = 5;
        price * qty
    };

    println!("resutl from multiply {result}");

}

fn main() {
    println!("Executing main....");

    primitive_data_types(false);
    
    compound_data_types(false);
    
    rust_function(true);
}
