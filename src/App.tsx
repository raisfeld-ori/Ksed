import { invoke } from "@tauri-apps/api/tauri";
import {useEffect} from "react";
import login from './pages/login/login';

function App() {
  const init = () => useEffect(() => {
    const initialize = async () => {await invoke("first_init", {});};
    initialize();
  }, []);
  init();

  return (login());
}

export default App;
