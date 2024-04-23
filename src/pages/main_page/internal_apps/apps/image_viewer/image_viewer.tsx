import { invoke } from "@tauri-apps/api";
import { AppInterface } from "../../App";
import App from "../../App";
import './image_viewer.css';
import { useState } from "react";

export default function image_viewer(file_selected: string | null) : AppInterface{
    const [image, set_image] = useState("");
    let update = async () => {
        if (file_selected == null) {return;}
        let name: string = await invoke('system_get', {key: 'name'});
        let password: string = await invoke('system_get', {key: 'password'});
        let bytes: string = await invoke('read_file', {name, password,file: file_selected});
        if (bytes == null) {return;}
        set_image(bytes);
    }
    let context_menu = <div>

    </div>
    let html = <div className="outer">
        {/*for now, this is readonly, cause it would take a while to remake this as a text editor*/}
        <img className="inner" src={image}>
        </img>
    </div>
    let [screen, set_display, fullscreen] = App(html, 'text editor');
    return {screen, set_display, fullscreen, context_menu, update};
}