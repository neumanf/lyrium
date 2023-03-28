import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [query, setQuery] = useState("");
  const [loading, setLoading] = useState(false);

  async function searchLyrics() {
    setLoading(true);
    setGreetMsg(await invoke<string>("search_lyrics", { query }));
    setLoading(false);
  }

  return (
    <div className="container">
      <div className="row">
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            await searchLyrics();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setQuery(e.currentTarget.value)}
            placeholder="Artist - song"
          />
          <button type="submit">Search</button>
        </form>
      </div>
      <pre>{loading ? "Loading..." : greetMsg}</pre>
    </div>
  );
}

export default App;
