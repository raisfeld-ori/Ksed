import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from '@tauri-apps/api/window';

function App() {
  const [count, setCount] = useState(0);
  appWindow.setFullscreen(true);

  async function incrementCount() {
    setCount(prevcount => prevcount + 1);
    await invoke("printf", {"text": "hello world"});
  };

  return (
    <div className="container">
      <button onClick={incrementCount}>Increment</button>
      <p>Count: {count}</p>
    </div>
  );
}

export default App;
