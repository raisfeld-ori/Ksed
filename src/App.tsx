import { invoke } from "@tauri-apps/api/tauri";
import {useEffect} from "react";
import login from './pages/login/login';
import signup from './pages/login/signup';
import loading from './pages/login/loading';
import { Route, Routes } from "react-router-dom";

function App() {
  const init = () => useEffect(() => {
    const initialize = async () => {await invoke("first_init", {});};
    initialize();
  }, []);
  init();

  function right_click(e: MouseEvent){
    // nothing in here currently
  }

  document.addEventListener('contextmenu', function(e) {
    e.preventDefault();
    right_click(e);
  });  

  return <Routes>
      <Route path="/" element={login()}></Route>
      <Route path="/signup" element={signup()}></Route>
      <Route path="/loading" element={loading()}></Route>
  </Routes>
}

export default App;
