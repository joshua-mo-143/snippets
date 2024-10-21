use database::{Database, SnippetDocument};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy};

#[derive(Clone)]
pub struct Tantivy {
    schema: Schema,
    index: Index,
    reader: IndexReader,
}

impl Tantivy {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut schema_builder = Schema::builder();

        schema_builder.add_i64_field("id", STORED | INDEXED);
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("description", TEXT | STORED);

        let schema = schema_builder.build();

        let index = Index::create_from_tempdir(schema.clone())?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        Ok(Self {
            schema,
            index,
            reader,
        })
    }

    pub async fn seed(&mut self, pool: &Database) -> Result<(), Box<dyn std::error::Error>> {
        let search_results = pool.retrieve_search_results().await?;

        for document in search_results {
            self.create_doc(document)?;
        }

        Ok(())
    }

    pub fn create_doc(
        &mut self,
        document: SnippetDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer: IndexWriter = self.index.writer(50_000_000)?;
        let id = self.schema.get_field("id")?;
        let title = self.schema.get_field("title")?;
        let desc = self.schema.get_field("description")?;

        let mut doc = TantivyDocument::default();

        doc.add_i64(id, document.id());
        doc.add_text(title, document.title());
        doc.add_text(desc, document.desc());

        writer.add_document(doc)?;

        writer.commit()?;

        Ok(())
    }

    pub fn fetch_docs(
        &self,
        params: FetchParams,
    ) -> Result<Vec<SnippetDocument>, Box<dyn std::error::Error>> {
        let searcher = self.reader.searcher();

        let id = self.schema.get_field("id")?;

        let title = self.schema.get_field("title")?;

        let desc = self.schema.get_field("description")?;

        let query_parser = QueryParser::for_index(&self.index, vec![id, title, desc]);

        let query = create_tantivy_query(params.query);
        let query = query_parser.parse_query(&query)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(params.results_num))?;

        let mut docs = Vec::new();
        top_docs.into_iter().for_each(|(_score, doc_addr)| {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_addr).unwrap();
            println!("{retrieved_doc:?}");
            let Some(OwnedValue::I64(num)) = retrieved_doc.get_first(id) else {
                panic!("Could not get ID from retrieved doc");
            };

            let Some(OwnedValue::Str(title)) = retrieved_doc.get_first(title) else {
                panic!("Could not get title from retrieved doc");
            };
            let Some(OwnedValue::Str(desc)) = retrieved_doc.get_first(desc) else {
                panic!("Could not get description from retrieved doc");
            };

            let doc = SnippetDocument {
                id: *num,
                title: title.to_owned(),
                description: desc.to_owned(),
            };

            docs.push(doc);
        });

        Ok(docs)
    }
}

fn create_tantivy_query(query: String) -> String {
    let str = query.split(" ").collect::<Vec<&str>>();

    let mut string = String::new();

    for (idx, search_term) in str.iter().enumerate() {
        let search_terms = format!("(title:{search_term} OR description:{search_term})");
        string.push_str(&search_terms);
        if idx < str.len() - 1 {
            string.push_str(" AND ")
        }
    }

    string
}

pub struct FetchParamsBuilder {
    query: Option<String>,
    results_num: Option<usize>,
}

impl FetchParamsBuilder {
    pub fn new() -> Self {
        Self {
            query: None,
            results_num: None,
        }
    }

    pub fn query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }

    pub fn results_num(mut self, results_num: usize) -> Self {
        self.results_num = Some(results_num);
        self
    }

    pub fn build(self) -> FetchParams {
        FetchParams::from(self)
    }
}

#[derive(Clone)]
pub struct FetchParams {
    query: String,
    results_num: usize,
}

impl Default for FetchParams {
    fn default() -> Self {
        Self {
            query: String::new(),
            results_num: 5,
        }
    }
}

impl FetchParams {
    pub fn builder() -> FetchParamsBuilder {
        FetchParamsBuilder::new()
    }
}

impl From<FetchParamsBuilder> for FetchParams {
    fn from(params: FetchParamsBuilder) -> Self {
        let query = params.query.unwrap();
        let results_num = params.results_num.unwrap_or(5);

        Self { query, results_num }
    }
}
