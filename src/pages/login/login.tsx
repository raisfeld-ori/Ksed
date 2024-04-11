import './login.css';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { useLocation, useNavigate } from 'react-router-dom';
import noenemies from '../main_page/assets/nature.jpg';
import exiticon from '../main_page/assets/x_icon.png';

function login(){
    const [name, setname] = useState("");
    const [password, setpassword] = useState("");
    const [error, seterror] = useState("");
    const navigate = useNavigate();
    const location = useLocation()
    useEffect(() => {
        if (location.state) {setname(location.state.name);setpassword(location.state.password);}
        if (location.state?.refresh) {window.location.reload();}
    }, [location.state]);

    async function authenticate(){
        if (name == '' || password == '') {seterror("your name/password can't be null");return;}
        let response = await invoke("authenticate_user", {name, password}).catch(e => {console.log(e)});
        if (response){
            navigate("/loading", {state: {name, password}});
        }
        else{
            seterror("failed to authenticate user");
        }
    }

        return <div id='background' onContextMenu={e => e.preventDefault()}>
                        <button className='upperbutton' aria-current={true} onClick={() => navigate("/", {state: {name, password}})}>Login</button>
            <button className='upperbutton' aria-current={false} onClick={() => navigate("/signup", {state: {name, password}})}>Sign Up</button>
        <img className='IhaveNoEnemys' src={noenemies} alt="backgroundimg" />
        <div className='iseewhoyouare' ></div>
        <button className='ignore' onClick={async () => {await invoke('close_app', {});}}>
                <img src={exiticon} alt="" />
            </button>
            <div id='menu'>

                <div id='form'>

                <h4 className="wordCarousel">
                <div> 
                    <ul className="flip5">
                        <li>olá 👋</li>
                        <li>mrhban 👋</li>
                        <li>Shalom 👋</li>
                        <li>Hola 👋</li>
                        <li>Hello 👋</li>
                    </ul>
                </div>
            </h4>

                <h1 className='login' >Welcome back!</h1>
                <p className='login2'>Please log to your account.</p>


                    <div className="input-group">
                    <label className="label">Username</label>
                     <input maxLength={16} onChange={e => setname(e.currentTarget.value)} autoComplete="off" name="info" id="info" className="input" placeholder='Enter your username' type="info" />
                    <div ></div>
                     </div>

                     <div className="input-group">
                    <label className="label">Password</label>
                     <input maxLength={16} onChange={e => setpassword(e.currentTarget.value)} 
                     onKeyDown={async e => {if (e.key == 'Enter') {await authenticate()}}} 
                     autoComplete="off" name="info" id="info" className="input" placeholder='Enter your password' type="password" />
                    <div ></div>
                     </div>

                     <button className="learn-more" onClick={authenticate}>
                        <span className="circle" aria-hidden="true">
                         <span className="icon arrow"></span>
                        </span>
                        <span className="button-text" onClick={authenticate}>Login</span>
                        </button>
                    <br />
                    <p id='error'>{error}</p>
                </div>
            </div>
        </div>
        
    }

export default login;