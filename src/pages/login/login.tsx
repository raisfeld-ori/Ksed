import './login.css';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { useNavigate } from 'react-router-dom';
import noenemies from '../main_page/assets/nature.jpg';
import arrowright from '../main_page/assets/arrowright.png';






function login(){
    const [name, setname] = useState("");
    const [password, setpassword] = useState("");
    const [error, seterror] = useState("");
    const navigate = useNavigate();

    async function authenticate(){
        let response = await invoke("authenticate_user", {name, password}).catch(e => {console.log(e)});
        console.log(response);
        if (response){
            navigate("/loading", {state: {name, password}});
        }
        else{
            seterror("failed to authenticate user");
        }
    }



        return <div id='background' onContextMenu={e => e.preventDefault()}>
                        <button className='upperbutton' aria-current={true} onClick={() => navigate("/")}>Login</button>
            <button className='upperbutton' aria-current={false} onClick={() => navigate("/signup")}>Sign Up</button>
        <img className='IhaveNoEnemys' src={noenemies} alt="backgroundimg" />
        <div className='iseewhoyouare' ></div>

            <div id='menu'>

                <div id='form'>


                <h1 className='login' >Welcome back!</h1>
                <p className='login2'>Please log to your account.</p>


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
                        <div className='dev'>
                            <img className='skippinclass' src={arrowright} alt="" onClick={() => navigate("/main_page")}/>
                        </div>
                    <br />
                    <p id='error'>{error}</p>




                </div>
            </div>
        </div>
        
    }

export default login;