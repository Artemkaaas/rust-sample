# Rust Sample
 
This is the simple Rust project demonstrating how to work with concurrency, borrowing, pattern matching and error handling.

- The program reads file names passed as arguments which this was started with.
- Parse them into AST using external `serde_json` crate.
- Merge them into the one common JSON.
- Each file will be processed in the separate thread.

File handling:
  1) accept only strings and nested objects as values.
  2) flatten view of nested objects in the result.

## Example

#### Input
Assume we have the following input files:
- file1.json {"name": "Artem"}
- file2.json {"second_name": "Ivanov", "passport": { "city": "Voronezh" }}
- file3.json {"status": "free"}

#### Execute
`cargo run file1.json file2.json file3.json`

#### Output
Prints common json: 
`{"name": "Artem", "second_name": "Ivanov", "city": "Voronezh", "status": "free" }`
    