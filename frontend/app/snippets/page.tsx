"use client";

import { NavigationBar } from "@/components/general/navBar";
import { Searchbar } from "@/components/general/searchBar";
import { SnippetList } from "@/components/snippets/snippetList";
import { useSnippets } from "@/lib/hooks/snippets";
import { useEffect } from "react";

export default function SearchPage() {
  const { snippets, fetch_snippets } = useSnippets();

  useEffect(() => {
    const fetchData = async () => {
      await fetch_snippets();
    };

    fetchData().catch(console.error);
  }, []);

  return (
    <main className="">
      <NavigationBar />
      <h1 className="">Snippets</h1>
      <Searchbar />
      {snippets.length > 0 ? <SnippetList snippets={snippets} /> : null}
    </main>
  );
}
