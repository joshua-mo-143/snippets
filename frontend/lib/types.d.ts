export interface Snippet {
  id: number;
  title: string;
  description: string;
  snippet: string;
}

export interface SnippetSearchResult {
  id: number;
  title: string;
  description: string;
}

export interface NewSnippet {
  title: string;
  description: string;
  snippet: string;
  language: string;
}
