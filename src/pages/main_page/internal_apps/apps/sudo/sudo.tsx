import { useEffect, useState } from "react";
import App, { AppInterface } from "../../App";
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";
import './sudo.css';


export default function sudo(text: string) : AppInterface{
    const [name, setname] = useState("");
    const [password, setpassword] = useState("");
    const [error, seterror] = useState("");
    const [succsess, set_succsess] = useState(false);
    const navigate = useNavigate();
    let authenticate = async () => {
        let current_user = await invoke('authenticate_user', {name, password});
        if (current_user){navigate("/loading", {state: {name, password}});seterror('successfully logged in!');set_succsess(true);}
        else{seterror('failed to authenticate user');set_succsess(false);}
    }
    let update = async () => {};
    let html = <div className="frame">
        <h1>{text}</h1>
        <label className="label">Name</label>
        <input type="text" onChange={e => setname(e.target.value)} className="input"/>
        <label className="label">Password</label>
        <input type="password" onKeyDown={async e => {if (e.key == 'Enter') {await authenticate()}}} 
        onChange={e => setpassword(e.target.value)} className="input"/>
        <button className="learn-more" onClick={authenticate}>
            <span className="circle" aria-hidden="true">
                <span className="icon arrow"></span>
            </span>
            <span className="button-text" onClick={authenticate}>Login</span>
        </button>
        <br></br>
        <p id="error" aria-current={succsess}>{error}</p>
    </div>
    let [screen, set_display, fullscreen] = App(html, 'sudo access');
    return {screen, set_display, fullscreen, context_menu: <div></div>, update};
}