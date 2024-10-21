interface SnippetProps {
  id: number | null;
  title: string;
  description: string;
}

export const SearchResult = ({ id, title, description }: SnippetProps) => {
  return (
    <div className="snippet-card">
      <h2>{title}</h2>
      <div className="snippet-info">
        <p> Description: {description} </p>
        <a href={`/snippet/view?id=${id}`}>Go to snippet</a>
      </div>
    </div>
  );
};

export default SearchResult;
