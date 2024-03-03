import './login.css';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { useNavigate } from 'react-router-dom';




function login(){
    const [name, setname] = useState("");
    const [password, setpassword] = useState("");
    const [error, seterror] = useState("");
    const navigate = useNavigate();

    async function authenticate(){
        let response = await invoke("authenticate_user", {name, password}).catch(() => {});
        if (response){
            navigate("/loading", {state: {name, password}});
        }
        else{
            seterror("failed to authenticate user");
        }
    }



        return <div id='background'>
            
        <div className='square'></div> 
        <button className='upperbutton' aria-current={true} onClick={() => navigate("/")}>Login</button>
                <button className='upperbutton' aria-current={false} onClick={() => navigate("/signup")}>Sign Up</button>
            <div className='triangle down'></div>   
            <div className='triangle left'></div>
            <div id='menu'>

                <div id='form'>

                    <h1 id='welcome_login'>Login</h1>
                    <p id='plslogin'>Welcome back! <td></td> please log to your account</p>

                    <div className="input-group">
                    <label className="label">Username</label>
                     <input onChange={e => setname(e.currentTarget.value)} autoComplete="off" name="info" id="info" className="input" placeholder='Enter your username' type="info" />
                    <div ></div>
                     </div>

                     <div className="input-group">
                    <label className="label">Password</label>
                     <input onChange={e => setpassword(e.currentTarget.value)} autoComplete="off" name="info" id="info" className="input" placeholder='Enter your password' type="info" />
                    <div ></div>
                     </div>
                    <button className='signup_login_button_main_menu' onClick={authenticate}>Log In</button>
                    <br />
                    <p id='error'>{error}</p>
                    
                </div>
            </div>
        </div>
        
    }

export default login;