"use client";
import { NavigationBar } from "@/components/general/navBar";
import { Snippet } from "@/lib/types";
import { useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";

async function fetch_snippet_by_id(id: number) {
  const api_url =
    process.env.NODE_ENV == "development"
      ? "http://127.0.0.1:8000"
      : `https://${process.env.NEXT_PUBLIC_API_URL}`;

  const res = await fetch(`${api_url}/api/snippets/${id}`);
  const data = await res.json();
  return data;
}

export default function SnippetPage() {
  const params = useSearchParams();
  const id = parseInt(params.get("id") as string);

  const [snippet, setSnippet] = useState<Snippet | null>(null);

  useEffect(() => {
    async function fetchData() {
      const snippet = await fetch_snippet_by_id(id);
      setSnippet(snippet);
    }
    fetchData();
  }, []);

  async function handleCopy(e: React.MouseEvent<HTMLButtonElement>) {
    if (snippet) {
      await navigator.clipboard.writeText(snippet?.snippet);

      const btn = e.target as HTMLButtonElement;
      btn.innerHTML = "Copied!";
    }
  }

  return (
    <main className="">
      <NavigationBar />
      <div className="view-snippet-container">
        <h1>View snippet</h1>
        <h2>Title: {snippet?.title}</h2>
        <div className="snippet-info">
          <p> Description: {snippet?.description} </p>
        </div>
        <pre>
          <button id={`button-${snippet?.id}`} onClick={handleCopy}>
            Copy
          </button>
          <code>{snippet?.snippet}</code>
        </pre>
      </div>
    </main>
  );
}
