//! `iife! {}` macro for a better immediately invoked function expression syntax
//!
//! # Example
//!
//! ```
//! use iife::iife;
//! use serde_json::{json, Value};
//!
//! fn request() -> Result<Value, String> {
//!     Ok(json!({
//!        "first": {
//!            "second": {
//!                "third": "value"
//!            }
//!        }
//!     }))
//! }
//!
//!
//! fn main() -> Result<(), String> {
//!     let response = request()?;
//!     
//!     // This iife context allows you to use ? to return an Option instead of Result
//!     let parsed = iife! {
//!         response.get("first")?.get("second")?.get("third")?.as_str()
//!     }
//!     .ok_or("Failed to parse")?;
//!
//!     assert_eq!(parsed, "value");
//!
//!     Ok(())
//! }
//!
//! ```

// Stolen from: https://users.rust-lang.org/t/writing-a-macro-rules-macro-that-takes-a-block-body/73539/10
#[macro_export]
macro_rules! iife {
    (@inner $s:stmt) => { $s };
    ( $($s:stmt)+ ) => {
        (||{
            $(iife!{@inner $s})+
        })()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    #[test]
    fn iife() {
        let some = iife! {
            Some(0)
        };
        assert_eq!(some, Some(0));
        let none: Option<i32> = iife! {
            None
        };
        assert_eq!(none, None);
    }

    fn ok_request() -> Result<Value, String> {
        Ok(json!({
           "first": {
               "second": {
                   "third": "value"
               }
           }
        }))
    }

    fn make_request_and_parse() -> Result<(), String> {
        let response = ok_request()?;

        let parsed = iife! {
            response.get("first")?.get("second")?.get("third")?.as_str()
        }
        .ok_or("Failed to parse")?;

        assert_eq!(parsed, "value");

        Ok(())
    }

    #[test]
    fn parsing() {
        let result = make_request_and_parse();
        assert!(result.is_ok());
    }
}
