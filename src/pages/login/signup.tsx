import './login.css';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api';
import noenemies from '../main_page/assets/nature2.jpg';


function Login() {
    const [name, setName] = useState("");
    const [password, setPassword] = useState("");
    const navigate = useNavigate();
    

    async function authenticate() {
        let user_exists = await invoke("user_exists", {name, password});
        if (!user_exists) {
            let name_val = await invoke('create_value', {valType: 'string', val: name});
            await invoke('system_make', {key: 'name',val: name_val});
            let password_val = await invoke('create_value', {valType: 'string', val: password});
            await invoke('system_make', {key: 'password',val: password_val});
            await invoke("create_user", {name, password})
            await invoke("save_user", {username: name, password});
            navigate("/loading", {state: {name, password}});
            
        }
    }

    return (
        <div id='background' onContextMenu={e => e.preventDefault()}>

            <button className='upperbutton' aria-current={false} onClick={() => navigate("/")}>Login</button>
            <button className='upperbutton' aria-current={true} onClick={() => navigate("/signup")}>Sign Up</button>
            <img className='IhaveNoEnemys' src={noenemies} alt="backgroundimg" />
        <div className='iseewhoyouare'></div>

            <div id='menu'>
                <div id='form'>

                    <h1 className='login' >Welcome!</h1>
                <p className='login2'>Enter your credentials to Sign Up</p>

                    <div className="input-group">
                        <label className="label">Username</label>
                        <input onChange={e => setName(e.currentTarget.value)} autoComplete="off" name="info" id="info" className="input" placeholder='Enter your username' type="info" />
                        <div></div>
                    </div>
                    <div className="input-group">
                        <label className="label">Password</label>
                        <input onChange={e => setPassword(e.currentTarget.value)} autoComplete="off" name="info" id="info" className="input" placeholder='Enter your password' type="info" />
                        <div></div>
                    </div>
                    <button className="learn-more" onClick={authenticate}>
                        <span className="circle" aria-hidden="true">
                            <span className="icon arrow"></span>
                        </span>
                        <span className="button-text">Sign Up</span>
                    </button>
                </div>
            </div>
        </div>
    );
}

export default Login;
