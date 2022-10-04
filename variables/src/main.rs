fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    // ^ updated a mutable value
    println!("The value of x is: {x}");

    let shadows = 5;
    let shadows = shadows + 1;
    // variables exist _separate_ from their 'code-names'
    // so there now exists (defacto) 'shadows$01' & 'shadows$02'
    // when calling "shadows" we will get the _most recent_,
    // which would be 'shadows$02' (after it's assigned)
    {
        let shadows = shadows * 2;
        // ^ here we've, effectively, made 'shadows$03'
        // it only lives in this scope, but will still be used below
        // as it's the most recent available
        // and when it fades (along with its scope) 'shadows$01' & 'shadows$02'
        // will continue to exist, unphased
        println!("The value of shadows in the inner scope is: {shadows}");
    }
    println!("The value of shadows in the outer scope is: {shadows}");


    // something basically unrelated to the above: Arrays
    // Array: fixed size, fixed (single) element type
    // elements called in 0-indexed fashion
    let arr: [isize; 3] = [-191; 3];
    println!("The 1st value of arr is: {}", arr[0]);
    println!("The 2nd value of arr is: {}", arr[1]);
    println!("The 3rd value of arr is: {}", arr[2]);
}
