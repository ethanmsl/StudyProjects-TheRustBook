//! Object Oriented Like Characteristics of Rust Ch. 17
//!

#![allow(unused_variables)]
#![allow(dead_code)]

/// A non-functioning, but organization suggestive library of code
/// for putting together a gui-library that could have the components it operates on
/// extended by way of users implementing defined traits on their
/// own objects/components.
mod gui {

    /// A trait-type (method-bound type?) that ensures drawability
    pub trait Draw {
        /// I suppose I'm based on side-effects and so have no out value (?)
        fn draw(&self);
    }

    /// A Vector of `Draw`able types
    /// (evidently all trait-type specifications require use of a pointer
    /// (and use the special `dyn` keyword)
    /// This contrats with the use of 
    /// ```ignore
    /// where
    ///    T: Draw,
    ///    ...
    /// ```
    /// in that it treats, properly (from the logic-abstraction perspective)
    /// the trait as a type, iteself -- and allows varied, more concrete, types
    /// to be mixed, single value to single value.
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
        //                     ^ this is **different** than using
        //                       `where T: Draw`
        //                       The diffrence is that `T` would have to be
        //                       a **single** type
        //                       , whereas <Box<dyn Draw>> can be *multiple*
        //                       concrete types (at different places in the same struct)
        //                       it just requires that they all
        //                       share the same **t-type**
        //                       [Sidenote: logically, (re: the logic-abstraction level)
        //                        Traits are just a "type",
        //                        Rust does not seem to treat them this way and, 
        //                        implementation wise there *may* be good reasons for
        //                        this -- however traits are just nodes and
        //                        functions/methods are just edges / morphisms
        //                        in a general network / pre-category]
        //
    }

    /// Do we get docs for an `impl` block???  
    /// *Hi, if so!*
    impl Screen {
        /// Draw all the members of the `components` field of a `Screen` object
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }



}
///////////////////////////////////////////////////////////////////////

/// Does this create documentation for the module `avg_cache`?  
/// (Yes! :)  
/// **Hi***!*
mod avg_cache {
    /// look at me, I'm documentation  
    /// I store a vector and an average of it's terms  
    /// *de facto* caching the average
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        /// ensures that the average field is always updated
        /// when vector elements are added
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
