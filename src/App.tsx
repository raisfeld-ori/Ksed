import { invoke } from "@tauri-apps/api/tauri";
import {useEffect} from "react";
import login from './pages/login/login';
import signup from './pages/login/signup';
import { Route, Routes } from "react-router-dom";
import Loading from "./pages/login/loading";
import main_page from './pages/main_page/main_page';

function App() {
  const init = () => useEffect(() => {
    const initialize = async () => {await invoke("first_init", {});};
    initialize();
  }, []);
  init();
  document.addEventListener('contextmenu', function(e) {e.preventDefault();});
  return <Routes>
      <Route path="/" element={login()}></Route>
      <Route path="/signup" element={signup()}></Route>
      <Route path="/loading" element={<Loading />}></Route>
      <Route path="/main_page" element={main_page()}></Route>
  </Routes>
}

export default App;
