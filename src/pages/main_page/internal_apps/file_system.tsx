import App from './App';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

function file_system(set_fs_html: React.Dispatch<React.SetStateAction<JSX.Element>>){
    const [location, set_location] = useState("");
    const [files, set_files] = useState([]);
    return () => {
    let app_html = <div className='file_system' onLoad={() => {console.log("test");}}>
        <h1></h1>
    </div>;
    let app = <App element={app_html} name='feet pics'/>;
    set_fs_html(app);
};
}

export default file_system;