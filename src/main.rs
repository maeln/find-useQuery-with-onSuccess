use std::{env, fs, path::Path};

use tree_sitter::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <typescript-file>", args[0]);
        std::process::exit(1);
    }

    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);
    let file_ext_opt = file_path.extension();

    if file_ext_opt.is_none() {
        eprintln!("Could find file extension");
        std::process::exit(1);
    }

    let file_ext = file_ext_opt.unwrap().to_str().unwrap();
    if file_ext[0..2] != *"ts" {
        eprintln!("{} is not a valid file extension", file_ext);
        std::process::exit(1);
    }

    let language: Language;
    if file_ext == "ts" {
        language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    } else {
        language = tree_sitter_typescript::LANGUAGE_TSX.into();
    }

    // Check if file exists
    if !file_path.exists() {
        eprintln!("Error: File '{}' not found", file_path_str);
        std::process::exit(1);
    }

    // Read file content
    let source_code = fs::read_to_string(file_path_str).expect("Could not read the file");

    // Initialize tree-sitter
    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("Could not load language parser");

    // Parse the source code
    let tree = parser
        .parse(&source_code, None)
        .expect("Failed to parse the source code");

    // Define a query to find useQuery function calls with onSuccess property
    let query_string = r#"
(call_expression
  function: (identifier) @function_name
  arguments: (arguments
    (_) ;; First argument (query key)
    (_) ;; Second argument (query function)
    (object
      (pair
        key: [(property_identifier) (string)] @key_name
      )
    ) @options_object
  )
  (#eq? @function_name "useQuery")
)
    "#;

    let query = Query::new(&language, query_string).expect("Failed to query the source");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    let mut found_res = false;
    while let Some(match_) = matches.next() {
        for capture in match_.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            if *capture_name != "options_object" {
                continue;
            }

            let node_text = &source_code[capture.node.byte_range()];

            if node_text.contains("onSuccess") {
                found_res = true;
            }
        }
    }

    if found_res {
        println!("{}", file_path_str);
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
