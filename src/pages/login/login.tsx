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
        let response = await invoke("authenticate_user", {name, password}).catch(e => {console.log(e)});
        
        if (response){
            navigate("/loading", {state: {name, password}});
        }
        else{
            seterror("failed to authenticate user");
        }
    }



        return <div id='background' onContextMenu={e => e.preventDefault()}>
            


                        <div className='triangle down'></div>
        <div className='triangle left'></div>
        <button className='upperbutton' aria-current={true} onClick={() => navigate("/")}>Login</button>
                <button className='upperbutton' aria-current={false} onClick={() => navigate("/signup")}>Sign Up</button>

            <div id='menu'>

                <div id='form'>

                    <p id='plslogin'>Welcome back! <td></td> Please log to your account.</p>


                    <div className="input-group">
                    <label className="label">Username</label>
                     <input maxLength={16} onChange={e => setname(e.currentTarget.value)} autoComplete="off" name="info" id="info" className="input" placeholder='Enter your username' type="info" />
                    <div ></div>
                     </div>

                     <div className="input-group">
                    <label className="label">Password</label>
                     <input maxLength={16} onChange={e => setpassword(e.currentTarget.value)} autoComplete="off" name="info" id="info" className="input" placeholder='Enter your password' type="info" />
                    <div ></div>
                     </div>

                     <button className="learn-more" onClick={authenticate}>
                        <span className="circle" aria-hidden="true">
                         <span className="icon arrow"></span>
                        </span>
                        <span className="button-text">Login</span>
                        </button>

                    <br />
                    <p id='error'>{error}</p>



                    <button className="btn-class-name" onClick={() => navigate("/main_page")}>
                    <span className="back"></span>
                    <span className="front"></span>
                    </button>
                    <h1 className='nuke' >NUKE AMERICA! <br /><span className='nuke2'>(and go to desktop)</span></h1>

                </div>
            </div>
        </div>
        
    }

export default login;