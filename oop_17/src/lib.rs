//! Object Oriented Like Characteristics of Rust Ch. 17
//!

#![allow(unused_variables)]
#![allow(dead_code)]

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
