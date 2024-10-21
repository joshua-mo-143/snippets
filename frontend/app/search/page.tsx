"use client";

import { NavigationBar } from "@/components/general/navBar";
import { Searchbar } from "@/components/general/searchBar";
import { SearchResults } from "@/components/snippets/searchResults";
import { useSnippets } from "@/lib/hooks/snippets";
import { SnippetSearchResult } from "@/lib/types";
import { useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";

export default function SearchPage() {
  const { search_snippets } = useSnippets();
  const [searchResults, setSearchResults] = useState<SnippetSearchResult[]>([]);
  const searchParams = useSearchParams();

  useEffect(() => {
    const fetchData = async () => {
      const query = searchParams.get("query") as string;

      const search_results = await search_snippets(query);
      setSearchResults(search_results);
    };

    fetchData().catch(console.error);
  }, []);

  return (
    <main className="">
      <NavigationBar />
      <h1 className="">Search</h1>
      <Searchbar />
      {searchResults.length > 0 ? (
        <SearchResults snippets={searchResults} />
      ) : null}
    </main>
  );
}
