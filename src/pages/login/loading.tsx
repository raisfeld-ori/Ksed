import './login.css';
import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import noenemies from '../main_page/assets/nature.jpg';
import pfp from './assets/Default_pfp.svg.png';
import { useNavigate } from 'react-router-dom';


// loading the page requires a username and a password
function loading(){
    const navigate = useNavigate();
    const location = useLocation();
    const [state, set_state] = useState("Loading");
    
    const [error, set_error] = useState("");
    if (!location.state){return <h1>DONT ACCESS THIS PAGE WITHOUT LOGGING IN</h1>}
    useEffect(() => {
        invoke("load_user", location.state).then(async result => {
            if (result) {set_state("ERROR");set_error("failed to load the user data");return;}
            let name_val = await invoke('create_value', {valType: 'string', val: location.state.name});
            await invoke('system_make', {key: 'name',val: name_val});
            let password_val = await invoke('create_value', {valType: 'string', val: location.state.password});
            await invoke('system_make', {key: 'password',val: password_val});
            navigate('/main_page');

        });
    }, []);

    return <div id='background' onContextMenu={e => e.preventDefault()}>
        <img className='IhaveNoEnemys' src={noenemies} alt="backgroundimg" />
        <div className='iseewhoyouare' ></div>


        <div className="dot">
            <img src={pfp} alt="Descriptive text" />
                </div>

                <p id='melody'>Welcome <span className='nameload'>{location.state.name}</span>!</p>

                <div className="load">{state}
                <p className='error'>{error}</p>
                <span></span>
                </div>
        </div>

}

export default loading;