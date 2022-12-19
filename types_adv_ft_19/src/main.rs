//! Advanced Types - Advanced Features Ch. 19

#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    println!("---------------------------------------------\n");

    // *******Type Alias********
    // merely syntactic convenience
    // a **single** type with an extra syntactic-handle
    // (useful if a type's default name is unwieldly)
    {
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x(raw_i32) + y(km) = {}", x + y);

        /// example unwieldly type being used
        /// (apparently "thunk" is a term for code to be evaluated later)
        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f1: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
        let f2: Thunk = Box::new(|| println!("hi"));

        fn takes_long_type_1(f: Box<dyn Fn() + Send + 'static>) {}
        fn takes_long_type_2(f: Thunk) {}

        fn returns_long_type_1() -> Box<dyn Fn() + Send + 'static> {
            Box::new(|| println!("hi"))
        }
        fn returns_long_type_2() -> Thunk {
            Box::new(|| println!("hi"))
        }

        // Type Alias with **Parmeter**
        {
            /// Example type alias that takes a paremeter;
            /// reducing `Result<...>`boilerplate
            type Result<T> = std::result::Result<T, std::io::Error>;
            pub trait Write {
                fn write(&mut self, buf: &[u8]) -> Result<usize>;
                fn flush(&mut self) -> Result<()>;

                fn  write_all(&mut self, buf: &[u8]) -> Result<()>;
                fn  write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()>;
            }
        }
    }
    println!("---------------------------------------------\n");
}
