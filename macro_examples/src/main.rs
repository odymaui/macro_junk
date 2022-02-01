extern crate tst_macros;

use tst_macros::make_answer;

//use tst_macros::make_derive_answer;
use tst_macros::AnswerFn;
use tst_macros::show_streams;
use tst_macros::return_as_is;


//https://doc.rust-lang.org/reference/procedural-macros.html
make_answer!();

#[allow(dead_code)]
#[derive(AnswerFn)]
struct Struct;

#[allow(dead_code)]
// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

#[allow(dead_code)]
// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

#[allow(dead_code)]
// Example: Multiple tokens in the input
#[show_streams(multiple => tokens)]
fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

#[allow(dead_code)]
// Example:
#[show_streams { delimiters }]
fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"

#[allow(dead_code)]
#[return_as_is]
fn test() {
    println!("Foo");
}

//https://doc.rust-lang.org/book/ch19-06-macros.html
//also:  https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html

struct Pancakes;

pub trait HelloMacro {
    fn hello_macro();
}

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

#[derive(tst_macros::HelloMacro)]
struct OatMeal;

#[derive(tst_macros::HelloMacro)]
struct FrenchToast;

#[derive(tst_macros::HelloMacro)]
struct Waffles;


macro_rules! Foo {

    () => { println!("macro_rules! can be in the same crate, Bar.") };
    ($ex:expr) => { println!("macro_rules! can be in the same crate, Bar. {} -> {}", stringify!($ex), $ex) }
}



macro_rules! adder {
    //note {{}} otherwise it won't compile...
    ($left:expr, $($right:expr),+) => {{

        let mut total:i32 = $left;
        $( 
            total += $right;
        )+
        total

    }}
}

macro_rules! csv_split {
    //note {{}} otherwise it won't compile...
    ($left:expr, $($right:expr),+) => {{

        println!("{}", $left);
        //without $()+ : error: variable 'right' is still repeating at this depth
        $(
            println!("{}", $right);
        )+

    }}
}

fn main() {


    //need at least two parameters.
    //adder!(1); //.expect("Need at least two values separated by a comma...");
    //can't expect as type returned by adder is number which isn't supported
    println!("Result: {}", adder!(1,4)); //.expect("Need at least two values separated by a comma...");
    println!("Result: {}", adder!(1,4,2)); //.expect("Need at least two values separated by a comma...");
    println!("Result: {}", adder!(1,4,2,3)); //.expect("Need at least two values separated by a comma...");
    println!("Result: {}", adder!(1,4,2, 4)); //.expect("Need at least two values separated by a comma...");

    println!("{}", answer());

    println!("{}", answer_derive());

    //manual
    Pancakes::hello_macro();

    //done by derive macro...
    OatMeal::hello_macro();
    FrenchToast::hello_macro();
    Waffles::hello_macro();

    Foo!();

    Foo!(true);
    Foo!(77 * 11);

    csv_split!("foo", "bar", "baz");
   
}

#[test]
fn it_works_make_answer() {
    
    let result = answer();
    
    assert_eq!(result, 42);
}

#[test]
fn it_works_make_answer_derive() {
    
    let result = answer_derive();
    
    assert_eq!(result, 42);
}

