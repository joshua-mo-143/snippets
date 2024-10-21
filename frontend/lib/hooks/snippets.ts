import { NewSnippet, Snippet } from "../types";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import { createJSONStorage } from "zustand/middleware";

interface SnippetState {
  snippets: Snippet[];
  add_snippet: (snippet: Snippet) => void;
  set_snippets: (snippet: Snippet[]) => void;
}

const useSnippetStore = create<SnippetState>()(
  persist(
    (set, _get) => ({
      snippets: [],
      add_snippet: (snippet: Snippet) =>
        set((state) => ({
          snippets: [snippet, ...state.snippets],
        })),
      set_snippets: (snippets: Snippet[]) =>
        set(() => ({ snippets: snippets })),
    }),
    {
      name: "snippet-storage",
      storage: createJSONStorage(() => sessionStorage),
    },
  ),
);

export function useSnippets() {
  const { snippets, add_snippet, set_snippets } = useSnippetStore();
  const api_url =
    process.env.NODE_ENV == "development"
      ? "http://127.0.0.1:8000"
      : `https://${process.env.NEXT_PUBLIC_API_URL}`;

  async function fetch_snippets() {
    const res = await fetch(`${api_url}/api/snippets`);
    const data = await res.json();
    set_snippets(data);
  }

  async function fetch_snippet_by_id(id: number) {
    const res = await fetch(`${api_url}/api/snippets/${id}`);
    const data = await res.json();
    return data;
  }

  async function search_snippets(text: string) {
    const res = await fetch(`${api_url}/api/search`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ query: text }),
    });
    const data = await res.json();
    return data;
  }

  async function create_snippet(snippet: NewSnippet) {
    try {
      await fetch(`${api_url}/api/snippets`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(snippet),
      });
    } catch (e: any) {
      console.log(e.message);
      throw new Error(`ERROR: ${e}`);
    }

    const id = snippets.length + 1;
    const returned_snippet: Snippet = {
      id: id,
      title: snippet.title,
      description: snippet.description,
      snippet: snippet.snippet,
    };

    add_snippet(returned_snippet);
  }

  return {
    fetch_snippets,
    fetch_snippet_by_id,
    search_snippets,
    create_snippet,
    snippets,
  };
}
