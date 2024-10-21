import { SnippetSearchResult } from "@/lib/types";
import SearchResult from "./searchResult";

interface Props {
  snippets: SnippetSearchResult[];
}

export const SearchResults = ({ snippets }: Props) => {
  return (
    <div className="snippet-list">
      {snippets.map((val) => (
        <SearchResult
          key={val.id}
          id={val.id}
          title={val.title}
          description={val.description}
        />
      ))}
    </div>
  );
};
