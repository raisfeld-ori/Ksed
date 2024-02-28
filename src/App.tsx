import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [count, setCount] = useState(0);
  async function init(){
    await invoke("printf", {"text": "test"});
    await invoke("first_init", {});
  }

  window.addEventListener('DOMContentLoaded', () => init);

  async function incrementCount() {
    setCount(prevcount => prevcount + 1);
  };

  return (
    <div className="container">
      <button onClick={init}>test</button>
      <button onClick={incrementCount}>Increment</button>
      <p>Count: {count}</p>
    </div>
  );
}

export default App;
