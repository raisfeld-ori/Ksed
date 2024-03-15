import './login.css';
import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import pfp from './assets/Default_pfp.svg.png';


// loading the page requires a username and a password
function loading(){
    const location = useLocation();
    const [state, set_state] = useState("Loading");
    
    const [error, set_error] = useState("");
    if (!location.state){return <h1>DONT ACCESS THIS PAGE WITHOUT LOGGING IN</h1>}
    useEffect(() => {
        invoke("load_user", location.state).then(result => {
            console.log("result: ", result);
            if (!result) {set_state("ERROR");set_error("failed to load the user data");return;}
        });
    }, []);

    return <div id='background' onContextMenu={e => e.preventDefault()}>

<div className='triangle down'></div>
        <div className='triangle left'></div>

        <div className="dot">
            <img src={pfp} alt="Descriptive text" />
                </div>

                <p id='melody'>Welcome {location.state.name}!</p>

                <div className="load">{state}
                <p className='error'></p>
                <span></span>
                </div>
        </div>

}

export default loading;