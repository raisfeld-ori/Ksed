import './login.css';
import { invoke } from '@tauri-apps/api';
// import { useNavigate } from 'react-router-dom';
import { useLocation } from 'react-router-dom';

// loading the page requires a username and a password
function loading(){
    const location = useLocation();
    if (!location.state){return <h1>DONT ACCESS THIS PAGE WITHOUT LOGGING IN</h1>}
    document.addEventListener('rust_event', () => {});
    invoke("update", {});
    invoke("update", {});
    return <div id='background'>
        <div id='user'>
            <h1>welcome {location.state.name}!</h1>
            <p>pfp will be here</p>
            <progress max={10} value={1}></progress>
        </div>
    </div>
}

export default loading;