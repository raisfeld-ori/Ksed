import { AppInterface } from "../../App";
import App from "../../App";
import './text_viewer.css';

export default function text_viewer(file_selected: string | null) : AppInterface{
    let update = async () => {

    }
    if (file_selected){
        
    }
    let context_menu = <div>

    </div>
    let html = <div className="outer">
        {/*for now, this is readonly, cause it would take a while to remake this as a text editor*/}
        <textarea className="inner" readOnly={true}>
            {file_selected}
        </textarea>
    </div>
    let [screen, set_display, fullscreen] = App(html, 'text editor');
    return {screen, set_display, fullscreen, context_menu, update};
}