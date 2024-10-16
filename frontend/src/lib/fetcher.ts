import { snippets } from "./snippets";
import type { Snippet } from "./types";

export const fetchSnippets = async (page: number = 0): Promise<Snippet[]> => {
  if (import.meta.env.MODE == "development") {
    return snippets.sort((a, b) => b.id - a.id) as Snippet[];
  } else {
    const res = await fetch("/api/snippets");
    const json = (await res.json()) as Snippet[];
    return json;
  }
};
