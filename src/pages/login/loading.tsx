import './login.css';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api';
// import { useNavigate } from 'react-router-dom';
import { useLocation } from 'react-router-dom';

// loading the page requires a username and a password
function loading(){
    const [value, setvalue] = useState(0);
    const location = useLocation();
    if (!location.state){return <h1>DONT ACCESS THIS PAGE WITHOUT LOGGING IN</h1>}
    setvalue(1);
    document.addEventListener('rust_event', () => {console.log("test, ", value)});
    invoke("update", {});
    invoke("update", {});
    return <div id='background'>
        {value}
        <div id='user'>
            <h1>welcome {location.state.name}!</h1>
            <p>pfp will be here</p>
            <progress max={10} value={1}>{value}</progress>
        </div>
    </div>
}

export default loading;