import App from './App';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

function file_system(set_fs_html: React.Dispatch<React.SetStateAction<JSX.Element>>){
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
    return () => {
        let Application = <div className='ApplicationDirectory'>
                    <h1 className='filesystemtxt2'>/{location}/</h1>
        </div>
            
    let app_html = <div className='frametest2'>
            {Application}


    </div>;
    let app = <App element={app_html} name='feet pics'/>;
    set_fs_html(app);
};
}

export default file_system;