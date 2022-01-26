extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn;

//https://doc.rust-lang.org/reference/procedural-macros.html
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(AnswerFn)]
pub fn make_derive_answer(_item: TokenStream) -> TokenStream {
    "fn answer_derive() -> u32 { 42 }".parse().unwrap()
}

//this runs during compilation as you see the output at build not when running.
#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //these don't expand as expected
    println!("as is: {}", stringify!(item));
    println!("as is with literal: {}", proc_macro::Literal::string(stringify!(item)));

    //this works as expected...
    println!("as is with literal to owned... {}", item.to_owned());

    println!("as is with literal to to_string... {}", item.to_string());
    item
}

//this runs during compilation as you see the output at build not when running.
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}


//https://doc.rust-lang.org/book/ch19-06-macros.html
//https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}