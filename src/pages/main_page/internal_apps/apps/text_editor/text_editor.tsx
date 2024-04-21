import { AppInterface } from "../../App";
import App from "../../App";

export default function text_editor(file_selected: string | null) : AppInterface{
    let update = async () => {

    }
    if (file_selected){
        
    }
    let context_menu = <div>

    </div>
    let html = <div>
        <textarea>
        </textarea>
    </div>
    let [screen, set_display, fullscreen] = App(html, 'text editor');
    return {screen, set_display, fullscreen, context_menu, update};
}