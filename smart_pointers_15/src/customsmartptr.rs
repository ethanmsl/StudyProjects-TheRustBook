//!  This file is for implementing a custom smart pointer
//!  whose, in turn, purpose is to demonstrate the use of
//!  the Droip method(trait).


/// Custom Smart Pointer
/// to demonstrate the use of drop
pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping Custom SmartPointer with data `{}`!", self.data);
    }
}


// impl CustomSmartPointer {
//     fn get_data(&self) -> &String {
//         &self.data
//     }
// }
//
// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping Custom SmartPointer with data `{}`!", self.get_data());
//     }
// }
