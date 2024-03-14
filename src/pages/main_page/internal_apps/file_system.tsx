import App, {State} from './App';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

function file_system() : [JSX.Element, React.Dispatch<React.SetStateAction<string>>]{
    const [location, set_location] = useState("Home");
    const [files, set_files] = useState([]);
    //@ts-expect-error
    invoke("ls", {}).then(result => set_files(result)).catch(console.log);
    //@ts-expect-error
    invoke("pwd", {}).then(result => set_location(result)).catch(console.log);

    const [menu_open, set_menu] = useState(false);
    const right_click = (ev: MouseEvent) => {
        //set_context({x: ev.clientX, y: ev.clientY});
        console.log("right clicked");
        //set_menu(true);

    }
    document.addEventListener('contextmenu', right_click);
    let Application = <div className='ApplicationDirectory'>
            <h1 className='filesystemtxt2'>/{location}/</h1>
        </div>   
    let app_html = <div className='frametest2'>
    {Application}
    </div>;
    const [display, set_display] = useState('none');
    let app = <App element={app_html} display={display} set_display={set_display} name='feet pics'/>;
    return [app, set_display];
}

export default file_system;