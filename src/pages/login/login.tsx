import './login.css';
import { window } from '@tauri-apps/api';

function login(){

    return <div id='background'>
        <button onClick={() => window.appWindow.setFullscreen(false)}>
            minimize the screen
        </button>
        <div className='triangle'></div>
    </div>
}

export default login;