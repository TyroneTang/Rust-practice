use std::{thread, time::Duration};
use chrono::{Datelike, Local, Weekday};

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


fn add(x: i32, y:i32) -> i32 {
    x * y
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

    println!("result from multiply {result}");

    let mod_add = add(50, 100);
    println!("check mod addd value result:: {mod_add}");

}

fn rust_ownership(is_enabled:bool) {
    if !is_enabled {
        println!("rust_ownership module is disabled! skipping...");
        return;
    }

    // owner s1, value is owned by s1
    let s1: String = String::from("RUST");
    let string_len = _calculate_string_len(&s1);
    println!("checkout the string length : {string_len}");

    println!("check prev owner s1: {s1}");

    let s2 = s1; // WARNING, ownership transfered, s2 is NEW owner. s1 is "gone";
    println!("check new owner s2: {s2}");

}

fn _calculate_string_len(input: &str) -> usize {
    // String is forced converted to &str. String > &str is compatible
    // &str > String is NOT compatible. One is a pointer accepted, the other references a heap allocated string

    // _calculate_string_len(literal);   // ✅ already &str
    // _calculate_string_len("inline");  // ✅ literal
    input.len()
}

fn _calculate_string_len_2(input: &String) -> usize {
    // still usable but beware below:
    // _calculate_string_len_2(literal);  // ❌ compile error — &str is not &String
    // _calculate_string_len_2("inline"); // ❌ compile error, need to do this instead _calculate_string_len_2(String::from("inline"))
    // use this if you intend to do copy operations, or capacity or mutate, else always use &str as
    // typing.
    input.len()
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // mutable since we want to update values
    fn withdraw(&mut self, amount: f64) {
        println!("widrawing {} from account {}", amount, self.owner);
        self.balance -= amount;
    }

    // immutable
    fn get_bal(&self) {
        println!("balance {} from account {}", self.balance, self.owner);
    }
}

fn borrowing_and_referencing(is_enabled: bool) {
    if !is_enabled {
        println!("borrowing_and_referencing module is disabled! skipping...");
        return;
    }

    // refrerences is similar to a pointer / same as borrowing
    // can be both immutable and mutable (will change the source value in owner)

    let x: i32 = 5;
    let _x_r: &i32 = &x;

    // mutable references
    let mut y: i32 = 20;
    let y_r: &mut i32 = &mut y;

    println!("chekout this value bef mutation {y_r}");
    // increment
    // & is the reference to the original var (a pointer, BUT not the value)
    // NOW TO ACCESS THAT VAR's VALUE, u need to get that value at that location, u need to deref it with * infront
    *y_r += 10;

    println!("value aft mutation 1 {y_r}");
    // println!("value aft mutation y 1 {y}");
    *y_r -= 5;
    println!("value aft mutation 2 {y_r}");

    // *y_r = 42;     // writes into y
    // *y_r += 1;     // mutates y in place
    // &*y_r          // re-borrows: gives you &i32 again (back to a reference)
    // The last line is the giveaway: if *y_r were already a reference, you wouldn't need to re-add & to get one. You add & because *y_r is the bare value/place, not a reference to it.

    // Quick mnemonic
    // & — make a reference (one level deeper into pointer-land)
    // * — follow a reference (one level back out, to the thing itself)
    // They're inverses. *&x and x mean the same thing.

    // engage in struct
    let mut balance: BankAccount = BankAccount{
        owner: String::from("new owner"),
        balance: 1000.00,
    };

    balance.get_bal();

    balance.withdraw(260.0);

    balance.get_bal();
     
}


fn variables_and_mutability(is_enabled: bool) {
    if !is_enabled {
        println!("variables_and_mutability module is disabled! skipping...");
        return;
    }

    let mut a: u16 = 5;
    println!("the value of a is {}", a);
    a = 20;
    println!("the value of a is {}", a);
}


const GLOBAL_PI: f64 = 3.14567;

fn constants(is_enabled: bool){
    if !is_enabled {
        println!("constants module is disabled! skipping...");
        return;
    }
    // const NOT MUTABLE and will never be mutable
    const _Y: i16 = 50;

    println!("const check global pi {}", GLOBAL_PI);
}


fn shadowing(is_enabled: bool) {
    if !is_enabled {
        println!("shadowing module is disabled! skipping...");
        return;
    }

    // Shadowing is when you declare a new variable with the same name as an existing one using let. The new binding hides (shadows) the old one for the rest of the scope. It's not mutation — it's a brand-new variable that happens to reuse the name.
    let x = 5;
    // shadowed, (use let)
    let x = "hello";

    let y = 20;

    {
        // new code block
        // shadowing = is now the new var until end of scope.

        // mostly used for defining, metadata of the var without having to give
        // additional variable name (e.g., num_len, ) then num = num.len()
        // put it in code blocks. no need to keep giving names since its temp for this block
        let x = x.len();
        // in py x_len = len(x)
        println!("length of x is {}", x);



        let y = y * 2;
        println!("in code block val of y {}", y);

        // y here has dropped out of scope.
    }
    println!("outside of code block val of x {}", x);
    println!("outside of code block val of y {}", y);

}

fn rust_loops(is_enabled: bool) {
    if !is_enabled {
        println!("rust_loops module is disabled! skipping...");
        return;
    }

    // loop similar to while True or in c++ loop {}
    let mut count = 0;

    loop {
        // until a break, infinite
        println!("looping... infinitely count: {}", count);

        thread::sleep(Duration::from_millis(100));

        count += 1;

        // break
        // if count > 20 {
        if count > 1 {
            println!("looping... count: {} is more than 20! breaking", count);
            break;
        }
    }

    let mut count = 0;

    // loop expression
    let result = loop {
        // if count > 30 {
        if count > 1 {
            // returns result on break
            break
            count * 100;
        }

        count += 1;
        println!("current loop 2 count {}", count);
        thread::sleep(Duration::from_millis(100));
    };

    println!("loop 2 count result {}", result);


    // nested while loop has labels and break can break out of everything or
    // go one level up.

    let condition = true;

    let now = Local::now();
    let day = now.weekday();

    println!("check day {}", day);

    while condition {
        if day == Weekday::Sun {
            println!("its sunday! break!");
            break;
        }
    };

    // for loops

    let arr: [i32; 6] = [1,2,3,4,5,6];

    // elements not index
    for el in arr {
        println!("check el of arr {}", el);
    };

}

fn rust_struct_topic(is_enabled: bool) {
    if !is_enabled {
        println!("rust_struct_topic module is disabled! skipping...");
        return;
    }

}

fn main() {
    println!("Executing main....");

    primitive_data_types(false);
    
    compound_data_types(false);
    
    rust_function(false);

    rust_ownership(false);

    borrowing_and_referencing(false);

    variables_and_mutability(false);

    constants(false);

    shadowing(false);

    rust_loops(false);

    rust_struct_topic(true);
}
