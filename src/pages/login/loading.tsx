import './login.css';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
import { useLocation } from 'react-router-dom';
import miku from '../../assets/Default_pfp.svg.png';


// loading the page requires a username and a password
function loading(){
    const [value, setvalue] = useState(0);
    const location = useLocation();
    if (!location.state){return <h1>DONT ACCESS THIS PAGE WITHOUT LOGGING IN</h1>}
    document.addEventListener('rust_event', () => {invoke("printf", {value: "yiipee"})});
    function run_loading(){invoke("load_user", location.state);}
    document.addEventListener('rust_event', () => {});
    invoke("user_", location.state);


    return <div id='background'>

<div className='triangle down'></div>
        <div className='triangle left'></div>

        <div className="dot">
            <img src={miku} alt="Descriptive text" />
                </div>

                <p id='melody'>Welcome {location.state.name}!</p>

                <div className="load">Loading
                <span></span>
                </div>
        </div>

}

export default loading;