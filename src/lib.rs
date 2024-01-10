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

    fn check_file(entry: std::fs::DirEntry) {
        use crate::idl::DocumentParser;
        let contents = std::fs::read_to_string(entry.path()).unwrap();
        let result = DocumentParser::new().parse(contents.as_str());
        println!("Result: {:?}", result);
        assert!(result.is_ok());
    }

    fn check_directory(entry: std::fs::DirEntry) {
        use std::fs;

        if entry.path().is_dir() {
            let paths = fs::read_dir(entry.path()).unwrap();
            for path in paths {
                //println!("Name: {}", path.unwrap().path().display())
                if let Ok(entry) = path {
                    check_directory(entry);
                }
            }
        } else {
            if entry
                .path()
                .extension()
                .unwrap_or_default()
                .eq_ignore_ascii_case("idl")
            {
                println!("Found idl: {}", entry.path().to_str().unwrap());
                check_file(entry);
            }
        }
    }

    #[test]
    fn test_dds_types_test_files() {
        use crate::idl::DocumentParser;
        use std::fs;
        let paths = fs::read_dir("./ext").unwrap();

        for path in paths {
            if let Ok(entry) = path {
                check_directory(entry);
            }
        }
    }
}
