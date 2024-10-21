import { Snippet } from "@/lib/types";
import { SnippetComponent } from "./snippet";

interface Props {
  snippets: Snippet[];
}

export const SnippetList = ({ snippets }: Props) => {
  return (
    <div className="snippet-list">
      {snippets.map((val) => (
        <SnippetComponent
          key={val.id}
          id={val.id}
          title={val.title}
          snippet={val.snippet}
          description={val.description}
        />
      ))}
    </div>
  );
};
