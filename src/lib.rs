#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub idl);

pub mod tree;

#[cfg(test)]
mod tests {
    use crate::idl::DocumentParser;

    #[test]
    fn test_parser() {
        let test_str = r#"
        
        // This is a line comment
        /* This is a block comment */

        /*
         * This is a multi-line comment
         */

        module Moos {
            // Test
            struct Message {
                double timestamp_123_test;
            };

        };

        "#;

        let result = DocumentParser::new().parse(test_str);
        println!("Result: {:?}", result);
        assert!(result.is_ok());
    }
}
