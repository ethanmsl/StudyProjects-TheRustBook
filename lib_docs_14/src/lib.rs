//! Look at *Me*  
//! I'm a library and get special treatment  
//! with regard to both *Docs* **and** *Testing*  
//! WoooOHeeeE  

/// Aha!
/// NOTE: declaring this as `pub` gets the doccomments to be used in documentation
/// ALSO NOTE: these '///' don't produce documentation for our `pub mod` declaration
pub mod helper;

/// juse here to use a helper file
///
/// # Examples
/// ...
pub fn use_helper() {
    helper::helper_pub();
}

/// adds left and right
///
/// # Examples
/// ```
/// let result = lib_docs_14::add(1, 2);
/// assert_eq!(3, result);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
