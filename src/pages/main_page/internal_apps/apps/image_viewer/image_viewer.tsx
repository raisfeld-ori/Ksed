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
        let bytes = await invoke('read_file', {name, password,file: file_selected});
        if (bytes == null) {return;}
        let data: [string, string] = await invoke('image_to_string', {bytes: bytes});
        let data_blob = new Blob([data[0]], {type: data[1]});
        const reader = new FileReader();
        reader.onload = (e: ProgressEvent<FileReader>) => {
            //for some reason, e.target? always returns, even if e.target != null
            console.log(e.target == null);
            console.log("before")
            if (e.target == null) {console.log("aaaaaahhhhh");return;}
            console.log("after");
            let result = e.target.result;
            console.log(result);
            if (typeof result === "string") {set_image(result);}
            console.log(image)
        };
        reader.onerror = (e) => {
            console.log("failed to read: ", e);
        }
        reader.readAsDataURL(data_blob);
    }
    let context_menu = <div>

    </div>
    let html = <div className="outer">
        <img className="inner" src={image}>
        </img>
    </div>
    let [screen, set_display, fullscreen] = App(html, 'image viewer');
    return {screen, set_display, fullscreen, context_menu, update};
}