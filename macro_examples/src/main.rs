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


struct Pancakes;


#[derive(tst_macros::HelloMacro)]
struct OatMeal;

#[derive(tst_macros::HelloMacro)]
struct FrenchToast;

#[derive(tst_macros::HelloMacro)]
struct Waffles;


pub trait HelloMacro {
    fn hello_macro();
}


impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}


fn main() {

    println!("{}", answer());

    println!("{}", answer_derive());

    //manual
    Pancakes::hello_macro();

    //done by derive macro...
    OatMeal::hello_macro();
    FrenchToast::hello_macro();
    Waffles::hello_macro();
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

