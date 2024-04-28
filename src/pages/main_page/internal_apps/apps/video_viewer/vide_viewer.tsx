import { invoke } from "@tauri-apps/api";
import { AppInterface } from "../../App";
import App from "../../App";
import './video_viewer.css';
import { useState } from "react";

export default function video_viewer(file_selected: string | null) : AppInterface{
    const [text, set_text] = useState("");
    let update = async () => {
        if (file_selected == null) {return;}
        let name: string = await invoke('system_get', {key: 'name'});
        let password: string = await invoke('system_get', {key: 'password'});
        let bytes = await invoke('read_file', {name, password,file: file_selected});
        if (bytes == null) {return;}
        let data: string = await invoke('bytes_to_string', {bytes});
        if (data == null) {return;}
        set_text(data);
    }
    let context_menu = <div>

    </div>
    let html = <div className="outer">
        {/*for now, this is readonly, cause it would take a while to remake this as a text editor*/}
        <video className="inner">
            <source src={text}></source>
        </video>
    </div>
    let [screen, set_display, fullscreen] = App(html, 'text viewer');
    return {screen, set_display, fullscreen, context_menu, update};
}