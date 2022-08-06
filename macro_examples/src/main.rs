extern crate tst_macros;


//use inline with tst_macros:: prefix to see local

//use tst_macros::make_answer;
//use tst_macros::make_derive_answer;
//use tst_macros::AnswerFn;
//use tst_macros::show_streams;
//use tst_macros::return_as_is;


//https://doc.rust-lang.org/reference/procedural-macros.html
tst_macros::make_answer!();

#[allow(dead_code)]
#[derive(tst_macros::AnswerFn)]
struct Struct;
//problem can only be derived once!
//error[E0428]: the name `answer_derive` is defined multiple times

/* 
#[allow(dead_code)]
#[derive(tst_macros::AnswerFn)]
struct Struct2;
*/


#[allow(dead_code)]
// Example: Basic function
#[tst_macros::show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

#[allow(dead_code)]
// Example: Attribute with input
#[tst_macros::show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

#[allow(dead_code)]
// Example: Multiple tokens in the input
#[tst_macros::show_streams(multiple => tokens)]
fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

#[allow(dead_code)]
// Example:
#[tst_macros::show_streams { delimiters }]
fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"

#[allow(dead_code)]
#[tst_macros::return_as_is]
fn test() {
    println!("Foo");
}

//https://doc.rust-lang.org/book/ch19-06-macros.html
//also:  https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html

struct Pancakes;

/*
macros can't be exported unless:
error: `proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]`
*/
pub trait HelloMacro {
    fn hello_macro();
}

//One way to generate a macro
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

//vs letting a single derive macro do that work!
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

//support various input formats 1, 2, or 3+
//note how it calls itself...
macro_rules! add{
    // first arm match add!(1,2), add!(2,3) etc
       ($a:expr,$b:expr)=>{
           {
               println!("Adding {} and {}", $a, $b);
               $a+$b
           }
       };
   // Second arm macth add!(1), add!(2) etc
       ($a:expr)=>{
           {
               println!("returning single item: {}", $a);
               $a
           }
       };
       
       ($a:expr,$($b:tt)*)=>{
       {
           println!("Spliting head $a:{} + rest: $b -> {}", $a, stringify!($($b)*));
          $a+add!($($b)*)
       }
       };
   }
   
   //modifies source variable which is input to this macro...
   macro_rules! add_other {
       ($id: ident, $value: expr ) => { 
           $id += $value; 
       };
       // for multiple argument
       ($id: ident, $( $value: expr), * ) => {
           $( $id += $value; )* 
       };
   }


fn main() {


    //need at least two parameters.
    //adder!(1); //.expect("Need at least two values separated by a comma...");
    //can't expect as type returned by adder is number which isn't supported
    println!("Result: {}", adder!(1,4)); //.expect("Need at least two values separated by a comma...");
    println!("Result: {}", adder!(1,4,2)); //.expect("Need at least two values separated by a comma...");
    println!("Result: {}", adder!(1,4,2,3)); //.expect("Need at least two values separated by a comma...");
    println!("Result: {}", adder!(1,4,2, 4)); //.expect("Need at least two values separated by a comma...");

    //proc macro and proc macro derive were run above, these
    //functions are available to call and run
    println!("calling proc macro generated function with result: {}", answer());
    println!("calling proc macro derive generated function with result: {}", answer_derive());

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

    //handle 1, 2, or 3+ arguments    
    println!("2=>{}", add!(2));
    println!("1+2=>{}", add!(1,2));
    println!("1,2,8,3,4,5=>{}", add!(1,2,8,3,4,5));

    //other function stuff

    let mut a: f32 = 1.1;
    let b: f32 = 4.1;
    //doesn't work if not a float!
    //error[E0277]: cannot add-assign `{integer}` to `f32`
    //($id: ident, $value: expr ) => { $id += $value; }
    //                                     ^^ no implementation for `f32 += {integer}`
    //add_other!(a, 4);

    println!("a before {}",a);

    //single not defined so not available...
    //add_other!(a);

    add_other!(a, 4f32);
    println!("a after {}",a);
    add_other!(a, b);
    println!("a after next -> {}",a);
    add_other!(a, b, 5.1, 7.1);
    println!("a after next 2 -> {}",a);
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

