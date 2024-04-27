import { invoke } from "@tauri-apps/api";
import { AppInterface } from "../../App";
import App from "../../App";
import './image_viewer.css';
import { useEffect, useState } from "react";

export default function image_viewer(file_selected: string | null) : AppInterface{
    const [image, set_image] = useState("");
    let update = async () => {
        if (file_selected == null) {return;}
        let name: string = await invoke('system_get', {key: 'name'});
        let password: string = await invoke('system_get', {key: 'password'});
        let bytes = await invoke('read_file', {name, password,file: file_selected});
        if (bytes == null) {return;}
        // let data: [string, string] = await invoke('image_to_string', {bytes: bytes});
        set_image(`${bytes}`);
    }
    useEffect(() => {console.log(image)}, [image]);
    let context_menu = <div>

    </div>
    let html = <div className="outer">
        <img className="inner" src={image}>
        </img>
    </div>
    let [screen, set_display, fullscreen] = App(html, 'image viewer');
    return {screen, set_display, fullscreen, context_menu, update};
}