import './login.css';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api';
import noenemies from '../main_page/assets/nature2.jpg';


function Login() {
    const [name, setName] = useState("");
    const [password, setPassword] = useState("");
    const [error, seterror] = useState("");
    const navigate = useNavigate();
    async function authenticate() {
        if (name == '' || password == '') {seterror("your name/password can't be null");return;}
        let user_exists = await invoke("user_exists", {name, password});
        if (!user_exists) {
            await invoke("create_user", {name, password})
            await invoke("save_user", {username: name, password});
            navigate("/loading", {state: {name, password}});
        }
        else{
            seterror('a user with this name already exists, please try a different');
        }
    }

    return (
        <div id='background' onContextMenu={e => e.preventDefault()}>

            <button className='upperbutton' aria-current={false} onClick={() => navigate("/", {state: {name, password}})}>Login</button>
            <button className='upperbutton' aria-current={true} onClick={() => navigate("/signup", {state: {name, password}})}>Sign Up</button>
            <img className='IhaveNoEnemys' src={noenemies} alt="backgroundimg" />
        <div className='iseewhoyouare'></div>

            <div id='menu'>
                <div id='form'>

                <h4 className="wordCarousel">
                <div> 
                    <ul className="flip5">
                        <li>Bem-vindo 👋</li>
                        <li>mrhbaan 👋</li>
                        <li>Baroh Aba 👋</li>
                        <li>bienvenidas 👋</li>
                        <li>Welcome 👋</li>
                    </ul>
                </div>
            </h4>

                    <h1 className='login' >Welcome!</h1>
                <p className='login2'>Enter your credentials to Sign Up</p>

                    <div className="input-group">
                        <label className="label">Username</label>
                        <input onChange={e => setName(e.currentTarget.value)} autoComplete="off" name="info" id="info" className="input" 
                        placeholder='Enter your username' type="info" />
                        <div></div>
                    </div>
                    <div className="input-group">
                        <label className="label">Password</label>
                     <input maxLength={16} onChange={e => setPassword(e.currentTarget.value)} 
                     onKeyDown={async e => {if (e.key == 'Enter') {await authenticate()}}}
                     autoComplete="off" name="info" id="info" className="input" placeholder='Enter your password' type="password" />
                        <div></div>
                    </div>
                    <button className="learn-more" onClick={authenticate}>
                        <span className="circle" aria-hidden="true">
                            <span className="icon arrow"></span>
                        </span>
                        <span className="button-text">Sign Up</span>
                    </button>
                    <p id='error'>{error}</p>
                </div>
            </div>
        </div>
    );
}

export default Login;
