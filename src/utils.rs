pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greet(name: Option<&str>) -> String {
    if let Some(name) = name
        && !name.is_empty()
    {
        format!("Hello {name}")
    } else {
        String::from("Hello")
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    #[test]
    fn test_add_simple() -> Result<()> {
        // -- Setup & Fixtures
        // ... here the code that preps/sets the context for the tests
        let a = 1;
        let b = 2;

        // -- Exec
        // ... here the code that executes the function to be tested
        let res = add(a, b);

        // -- Check
        // ... here all of the checks/asserts
        // ... can be commented like `// check the blocks` and multiple lines below
        assert_eq!(res, 3);
        Ok(())
    }

    #[test]
    fn test_greet_simple() -> Result<()> {
        // -- Setup & Fixtures
        let name = "Alice";

        // -- Exec
        let msg = greet(Some(name));

        // -- Check
        let expected = "Hello Alice";
        assert_eq!(msg, expected);

        Ok(())
    }

    #[test]
    fn test_greet_empty() -> Result<()> {
        // -- Exec & Check
        let out = greet(Some(""));
        assert_eq!(out, "Hello");
        Ok(())
    }

    // region:    --- Support
    // ... support functions that might be used in above code.
    // endregion: --- Support
}

// endregion: --- Tests
