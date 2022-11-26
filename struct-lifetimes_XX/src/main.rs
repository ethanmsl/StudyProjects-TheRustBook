#![allow(unused_variables)]
#![allow(dead_code)]

// Hmm... a post here
// (https://users.rust-lang.org/t/what-is-the-point-of-lifetime-parameters-in-struct-impl-blocks/14631/8)
// said that the code below wouldn't compile and I didn't see why ... but it does compile... (¿maybe
// it wouldn't in 2020?)
fn main() {
    struct Foo<'a> {
        x: &'a i32,
    }

    struct Bar<'a> {
        f1: &'a mut Foo<'a>,
        f2: &'a Foo<'a>,
    }

    fn main() {
        let x = &20;
        let mut foo = Foo { x };

        let x2 = &10;

        let mut foo2 = Foo { x: x2 };
        let bar = Bar {
            f1: &mut foo,
            f2: &foo2,
        };
    }
}
