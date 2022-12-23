use macros_adv_ft_19 as lib;

fn main() {
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

    // using our trait implementor macro
    {

    }
}
