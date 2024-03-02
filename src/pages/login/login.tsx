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
        <div className='triangle down'></div>
        <div className='triangle left'></div>
        <div id='menu'>
            <button className='upperbutton' aria-current={true} onClick={() => navigate("/")}>login</button>
            <button className='upperbutton' aria-current={false} onClick={() => navigate("/signup")}>sign up</button>

            <div id='form'>
                <h1>Welcome back!</h1>
                <p>please log in</p>
                <input onChange={e => setname(e.currentTarget.value)} placeholder='write your name'></input>
                <br />
                <input onChange={e => setpassword(e.currentTarget.value)} placeholder='write your password'></input>
                <br />
                <button onClick={authenticate}>log in</button>
                <br />
                <p id='error'>{error}</p>
            </div>
        </div>
    </div>
}

export default login;