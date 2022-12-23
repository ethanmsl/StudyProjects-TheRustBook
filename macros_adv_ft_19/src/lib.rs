#[macro_export]
macro_rules! vec_bespoke {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

///////////////////////////////////////////////////////////////

// use proc_macro;
//
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
// }

///////////////////////////////////////////////////////////////

// macro defined
// the macro derive is in a *seperate crate* within this crate's main directory
// (that crates is called "hello_macro_derive")
pub trait HelloMacro {
    fn hello_macro();
}
