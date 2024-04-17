import App, { AppInterface } from "../../App";


export default function Settings() : AppInterface{
    
    let app_html = <h1>
        test
    </h1>
    let context_menu = <h1></h1>
    let update = async () => {}
    let [screen, set_display] = App(app_html, "settings");
    return {screen, set_display, context_menu, update};
}

