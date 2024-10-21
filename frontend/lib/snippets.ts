export const snippets = [
  {
    id: 3,
    title: "Test snippet 3",
    description: "A test snippet for development mode",
    language: "rust",
    snippet:
      "\ntrait Serializable {\n     fn serialize(&self) -> String;\n}\n\nstruct MyStruct {\n    body: String,\n}\n\nimpl Serializable for MyStruct {\n    fn serialize(&self) -> String {\n        self.body.clone()\n    }\n}",
  },
  {
    id: 2,
    title: "Test snippet 2",
    description: "A test snippet for development mode",
    language: "rust",
    snippet: '\nfn test_snippet() {\n     println!("Hello world!");\n}',
  },
  {
    id: 1,
    title: "Test snippet 1",
    description: "A test snippet for development mode",
    language: "rust",
    snippet: '\nfn test_snippet() {\n     println!("Hello world!");\n}',
  },
];

export default snippets;
