import './login.css';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
// import { useNavigate } from 'react-router-dom';
import { useLocation } from 'react-router-dom';
import miku from '../../assets/alpha.png';


// loading the page requires a username and a password
function loading(){
    const [value, setvalue] = useState(0);
    const location = useLocation();
    if (!location.state){return <h1>DONT ACCESS THIS PAGE WITHOUT LOGGING IN</h1>}
    document.addEventListener('rust_event', () => {});
    invoke("user_", location.state);


    return <div id='background'>

<div className='triangle down'></div>
        <div className='triangle left'></div>
        <div id='user'>
            <p>{value}</p>
            <h1>welcome {location.state.name}!</h1>

            <div className="dot">
            <img src={miku} alt="Descriptive text" />
                </div>
                
                <h1>i love you {location.state.name}!</h1>
            <progress max={10} value={1}></progress>
        </div>
    </div>
}

export default loading;