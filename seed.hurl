POST http://localhost:8000/api/snippets
{
    "title": "Test snippet",
    "description": "A test snippet for testing Rust",
    "snippet": "\nfn test_snippet() {\n    println!(\"Hello world!\");\n}",
    "language":"rust"
}
