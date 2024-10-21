"use client";
import { useRouter, useSearchParams } from "next/navigation";
import { useState } from "react";

export function Searchbar() {
  const params = useSearchParams();
  const router = useRouter();
  const [query, setQuery] = useState<string>(params.get("query") ?? "");

  function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const url = `/search?query=${encodeURI(query)}`;
    router.push(url);
  }

  function handleInput(e: React.ChangeEvent<HTMLInputElement>) {
    e.preventDefault();

    setQuery(e.target.value);
  }

  return (
    <div className="searchbar">
      <form onSubmit={handleSubmit}>
        <input
          name="query"
          required
          type="text"
          value={query}
          onChange={handleInput}
        />
        <button type="submit">Search</button>
      </form>
    </div>
  );
}
