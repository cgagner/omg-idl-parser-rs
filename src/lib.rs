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

        module Moos {
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
