interface SnippetProps {
  id: number;
  title: string;
  description: string;
  snippet: string;
}

export const SnippetComponent = ({
  id,
  title,
  description,
  snippet,
}: SnippetProps) => {
  async function handleCopy(e: React.MouseEvent<HTMLButtonElement>) {
    await navigator.clipboard.writeText(snippet);

    const btn = e.target as HTMLButtonElement;
    btn.innerHTML = "Copied!";
  }

  return (
    <div className="snippet-card">
      <h2>{title}</h2>
      <div className="snippet-info">
        <p> Description: {description} </p>
      </div>
      <pre>
        <button id={`button-${id}`} onClick={handleCopy}>
          Copy
        </button>
        <code>{snippet}</code>
      </pre>
    </div>
  );
};

export default SnippetComponent;
