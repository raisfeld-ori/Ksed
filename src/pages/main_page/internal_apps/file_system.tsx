import App from './App';
import { invoke } from '@tauri-apps/api';
import { useState, useEffect } from 'react';

function file_system() : [JSX.Element, React.Dispatch<React.SetStateAction<string>>, JSX.Element]{
    const [location, set_location] = useState("Home");
    const [files, set_files] = useState([]);
    //@ts-expect-error
    invoke("ls", {}).then(result => set_files(result)).catch(console.log);
    //@ts-expect-error
    invoke("pwd", {}).then(result => set_location(result)).catch(console.log);

    const [{dx, dy}, set_positions] = useState({dx: 0, dy: 0});
    const [ctx_display, set_ctx_display] = useState('none');
    useEffect(() => {
        document.addEventListener("click", () => set_ctx_display('none'));
        return () => document.removeEventListener("click", () => set_ctx_display('none'));
    }, [])

    const right_click = (ev: React.MouseEvent) => {
        //set_context({x: ev.clientX, y: ev.clientY});
        ev.preventDefault();
        console.log("right clicked");
        //set_menu(true);
        set_ctx_display('inherit');
        set_positions({dx: ev.clientX, dy: ev.clientY});
    }

    let context_menu = <div className='ContextMenu'
    style={{
        top: dy + 2 + 'px',
        left: dx + 2 + 'px',
        display: `${ctx_display}`,
    }}
    ><h1>test</h1><h1>test 2</h1></div>;
    let Application = <div className='ApplicationDirectory'>
            <h1 className='filesystemtxt2'>/{location}/</h1>
        </div>
    let app_html = <div className='frametest2' onContextMenu={right_click}>
    {Application}
    </div>;
    const [display, set_display] = useState('none');
    let app = <App element={app_html} display={display} set_display={set_display} name='File System'/>;
    return [app, set_display, context_menu];
};


export default file_system;