import { invoke } from "@tauri-apps/api";
import { AppInterface } from "../../App";
import App from "../../App";
import './text_viewer.css';
import { useState } from "react";

export default function text_viewer(file_selected: string | null) : AppInterface{
    const [text, set_text] = useState("");
    let update = async () => {
        if (file_selected == null) {return;}
        let name: string = await invoke('system_get', {key: 'name'});
        let password: string = await invoke('system_get', {key: 'password'});
        let data: string = await invoke('read_file', {file: file_selected, name, password});
        // i'l work on it later, idk what to do here for now
        console.log(data);
        if (data == null) {return;}
        set_text(data);
    }
    let context_menu = <div>

    </div>
    let html = <div className="outer">
        {/*for now, this is readonly, cause it would take a while to remake this as a text editor*/}
        <textarea className="inner" readOnly={true} value={text}>
        </textarea>
    </div>
    let [screen, set_display, fullscreen] = App(html, 'text editor');
    return {screen, set_display, fullscreen, context_menu, update};
}