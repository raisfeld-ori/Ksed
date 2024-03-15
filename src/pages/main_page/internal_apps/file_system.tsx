import App, {State} from './App';
import { invoke } from '@tauri-apps/api';
import { useState, useEffect } from 'react';

function file_system() : [JSX.Element, React.Dispatch<React.SetStateAction<string>>]{
    const [location, set_location] = useState("Home");
    const [files, set_files] = useState([]);
    //@ts-expect-error
    invoke("ls", {}).then(result => set_files(result)).catch(console.log);
    //@ts-expect-error
    invoke("pwd", {}).then(result => set_location(result)).catch(console.log);

    const [{dx, dy}, set_positions] = useState({dx: 0, dy: 0});
    const [ctx_display, set_ctx_display] = useState('none');
    const right_click = (ev: MouseEvent) => {
        //set_context({x: ev.clientX, y: ev.clientY});
        console.log("right clicked");
        //set_menu(true);
        set_ctx_display('inherit');
        set_positions({dx: ev.clientX, dy: ev.clientY});
    }
    const left_click = () =>{
        if (ctx_display) {set_ctx_display('none');}
    }
    useEffect(() => {
        document.addEventListener('contextmenu', right_click);
        document.addEventListener('click', left_click);
        return () => {
            document.removeEventListener('contextmenu', right_click);
            document.removeEventListener('click', left_click);
        };
    }, []); 
    let Application = <div className='ApplicationDirectory'>
            <h1 className='filesystemtxt2'>/{location}/</h1>

            <div className='ContextMenu'
            style={{
                transform: `translate3d(${dx}px, ${dy}px, 0)`,
                display: `${ctx_display}`,
            }}
            ><h1>test</h1></div>
        </div>   
    let app_html = <div className='frametest2'>
    {Application}
    </div>;
    const [display, set_display] = useState('none');
    let app = <App element={app_html} display={display} set_display={set_display} name='feet pics'/>;
    return [app, set_display];
}

export default file_system;