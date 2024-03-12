import App from './App';
import { invoke } from '@tauri-apps/api';

function file_system(set_fs_html: React.Dispatch<React.SetStateAction<JSX.Element>>){
    return () => {
    let app_html = <h1 onClick={async () => {console.log(await invoke("list_commands", {}));}}>here is some code</h1>;
    let app = <App element={app_html} name='feet pics'/>;
    set_fs_html(app);
};
}

export default file_system;