import './login.css';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
// import { useNavigate } from 'react-router-dom';
import { useLocation } from 'react-router-dom';

// loading the page requires a username and a password
function loading(){
    const [value, setvalue] = useState(0);
    const location = useLocation();
    if (!location.state){return <h1>DONT ACCESS THIS PAGE WITHOUT LOGGING IN</h1>}
    document.addEventListener('rust_event', () => {invoke("printf", {value: "yiipee"})});
    function run_loading(){invoke("load_user", location.state);}
    return <div id='background'>
        <div id='user'>
            <p>{value}</p>
            <h1>welcome {location.state.name}!</h1>
            <p>pfp will be here</p>
            <button onClick={run_loading} className='upperbutton'>start loading</button>
            <progress max={10} value={1}></progress>
        </div>
    </div>
}

export default loading;