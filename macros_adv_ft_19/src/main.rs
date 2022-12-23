use macros_adv_ft_19 as lib;
use lib::HelloMacro;  // where we defined trait (which, alas, has "macro" in its name)
use hello_macro_derive_19::HelloMacro;  // where we defined macro trait deriver
//                          ^ that I'm calling this rather than the
//                            `hello_macro_derive` surprises me...
//                            I suppose it's some nominal convenience-oriented peculiarity

#[derive(HelloMacro)]
/// unit struct
struct Pancakes;

fn main() {
    println!("---------------------------------------------\n");
    // using our "declarative" (pattern match and replace) macro
    {
        let v = lib::vec_bespoke![1, 2, 3];
        println!("v = {:?}", v);

        // let v = lib::vec_bespoke![1, 2,,, 3,];
        // //                             ^ not sure why that's not allowed since we used
        // //                               `,*` in the macro...
        // //                               ^ ANSWER (maybe): because "separator"s are
        // //                                 treated specially.
        // //                            The `*` applies to the whole `(...),` next to the `*`
        // //                            and the `,` just means ~"and allow comma as a separtor"
        // //                            and it *seems* to have special rules that allow classic
        // //                            sepparator usage (with*out* trailing separators)
        // println!("v = {:?}", v);

        // let v = lib::vec_bespoke![1, 2, 3,];
        // //                               ^ the default macro sepperator rules **don't** allow for
        // //                                 trailing separators
        // println!("v = {:?}", v);
    }
    println!("---------------------------------------------\n");

    // using our "HelloMacro" trait implementor macro
    //             ^ note that's the name of the *trait*, not the macro
    //               (questionable naming scheme, but I'm not going to go back
    //                and repair it all right now)
    {
        // let immapancake = Pancakes;
        // immapancake.hello_macro();
        Pancakes::hello_macro();
        // ^ the function is for the struct itself, rather than its instances
    }
}
