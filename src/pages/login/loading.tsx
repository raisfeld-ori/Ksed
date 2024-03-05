import './login.css';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
// import { useNavigate } from 'react-router-dom';
import { useLocation } from 'react-router-dom';
import miku from '../../assets/alpha.png';
import pullup from '../../assets/Okiupp.mp3';
import okpullup from '../../assets/OK-I-PULL-UP.ogg';


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
                <h1>kys</h1>
            <progress className='rotating' max={10} value={1}></progress>
                <p id='melody'>enjoy this melody while ori is fixing the progress bar</p>
            <audio controls autoPlay>
             <source src={okpullup} type="audio/ogg" />
             <source src={pullup} type="audio/mpeg" />
             Your browser does not support the audio element.
            </audio>

        </div>
    </div>
}

export default loading;