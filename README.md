`iife! {}` macro for a better immediately invoked function expression syntax

# Example

```rust
use iife::iife;
use serde_json::{json, Value};

fn request() -> Result<Value, String> {
    Ok(json!({
       "first": {
           "second": {
               "third": "value"
           }
       }
    }))
}

fn main() -> Result<(), String> {
    let response = request()?;
    
    // This iife context allows you to use ? to return an Option instead of Result
    let parsed = iife! {
        response.get("first")?.get("second")?.get("third")?.as_str()
    }
    .ok_or("Failed to parse")?;
    assert_eq!(parsed, "value");
    Ok(())
}
```
