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
    let ref_int: &i32 = &20; 

    // heap-allocated integer (dynamic) & copied
    let al_str: String = String::from("hello!!!");
    // stack-allocated referenced(pointer) to an string in binary
    let ref_str: &str = &"abc";
}


fn main() {
    println!("Executing main....");

    primitive_data_types(false);
    compound_data_types(true);
}
