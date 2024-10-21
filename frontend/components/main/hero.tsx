import { NavigationBar } from "../general/navBar";
import { Searchbar } from "../general/searchBar";

export function Hero() {
  return (
    <section className="hero">
      <NavigationBar />
      <h1>Rustytips</h1>
      <p>The easiest way to store and share your code snippets.</p>
      <Searchbar />
    </section>
  );
}
