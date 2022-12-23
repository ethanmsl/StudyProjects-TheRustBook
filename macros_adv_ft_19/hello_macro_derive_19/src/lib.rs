use proc_macro::TokenStream;  // built-in; prob. required the `proc-macro` flag in toml
use quote::quote;  // (rust AST ~~> tokenstream)
use syn;  // Parsing library (rust tokenstream ~~> AST)

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a represenation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    //Build the trait implementation
    impl_hello_macro(&ast)
}


// # Using the #[derive(...)] on an example struct
// ## an example AST structure built from the tokenstream
//
// #[derive(HelloMacro)]
// struct Pancakes;
//
// ~~~>
//
// DeriveInput {
//     // --snip--
//
//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
                //                                         ^ stringify!() is a macro that
                //                                           converts an expression into a
                //                                           string literal
                //                          Question: is this comment technically
                //                          getting built as part of the derive macro?
                //                          And would there be a way for me to see that
                //                          since for (at least) many build set-ups the
                //                          compiler would strip out the comments...?
            }
        }
    };
    gen.into()
}
