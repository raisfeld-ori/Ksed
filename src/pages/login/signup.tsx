import './login.css';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';


function login(){
    const [name, setname] = useState("");
    const [password, setpassword] = useState("");
    const navigate = useNavigate();

    function authenticate(){
        console.log(name, ", ", password);
    }

    return <div id='background'>
        <div className='triangle down'></div>
        <div className='triangle left'></div>
        <div id='menu'>
        <button className='upperbutton' aria-current={false} onClick={() => navigate("/")}>login</button>
            <button className='upperbutton' aria-current={true} onClick={() => navigate("/signup")}>sign up</button>

            <div id='form'>
                <h1>Welcome!</h1>
                <p>Create an account</p>
                <input onChange={e => setname(e.currentTarget.value)}></input>
                <br />
                <input onChange={e => setpassword(e.currentTarget.value)}></input>
                <br />
                <button onClick={authenticate} id='login'>sign up</button>
            </div>
        </div>
    </div>
}

export default login;