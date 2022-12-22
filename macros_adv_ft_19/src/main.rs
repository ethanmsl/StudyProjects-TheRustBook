
use macros_adv_ft_19 as lib;

fn main() {
    let v = lib::vec_bespoke![1, 2, 3];
    println!("v = {:?}", v);

    // let v = lib::vec_bespoke![1, 2,,, 3,];
    // //                             ^ not sure why that's not allowed since we used
    // //                               `,*` in the macro...
    // println!("v = {:?}", v);

}
