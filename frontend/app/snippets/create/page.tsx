"use client";
import { useSnippets } from "@/lib/hooks/snippets";
import { NewSnippet } from "@/lib/types";
import { ChangeEvent, useState } from "react";
import { useRouter } from "next/navigation";
import { NavigationBar } from "@/components/general/navBar";

export default function CreateSnippetForm() {
  const { create_snippet } = useSnippets();
  const router = useRouter();

  const [error, setError] = useState<string>("");

  const [formData, setFormData] = useState<NewSnippet>({
    title: "",
    snippet: "",
    description: "",
    language: "",
  });

  const handleTextInput = (
    e:
      | ChangeEvent<HTMLInputElement>
      | ChangeEvent<HTMLTextAreaElement>
      | ChangeEvent<HTMLSelectElement>,
  ) => {
    const { name, value } = e.target;
    setFormData({
      ...formData,
      [name]: value,
    });
  };

  async function handleSubmit(e: React.SyntheticEvent) {
    e.preventDefault();

    try {
      await create_snippet(formData);
      console.log("Successfully sent!");
      router.push("/");
    } catch (e: any) {
      setError(e.message);
    }
  }

  return (
    <main>
      <NavigationBar />
      <h1>Submit a Snippet</h1>
      <form onSubmit={(e) => handleSubmit(e)}>
        <div className="error">{error}</div>
        <label>
          <p>Title:</p>
          <input
            name="title"
            required
            onChange={handleTextInput}
            value={formData.title}
          />
        </label>
        <label>
          <p>Description:</p>
          <textarea
            name="description"
            cols={65}
            rows={5}
            value={formData.description}
            onChange={handleTextInput}
          ></textarea>
        </label>
        <label>
          <span>Language:</span>
          <select onChange={handleTextInput} value={formData.language}>
            <option value="rust">Rust</option>
          </select>
        </label>
        <label>
          <p>Snippet:</p>
          <textarea
            name="snippet"
            cols={65}
            rows={5}
            value={formData.snippet}
            onChange={handleTextInput}
          ></textarea>
        </label>
        <button className="submit-button" type="submit">
          Submit
        </button>
      </form>
    </main>
  );
}
