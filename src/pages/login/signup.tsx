import './login.css';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import miku from '../../assets/Default_pfp.svg.png';


function Login() {
    const [name, setName] = useState("");
    const [password, setPassword] = useState("");
    const navigate = useNavigate();

    function authenticate() {
        console.log(name, ", ", password);
    }

    return (
        <div id='background'>
            <div className='triangle down'></div>
            <div className='triangle left'></div>
            <button className='upperbutton' aria-current={false} onClick={() => navigate("/")}>Login</button>
            <button className='upperbutton' aria-current={true} onClick={() => navigate("/signup")}>Sign Up</button>
            <div id='menu'>
                <div id='form'>
                    <p id='plslogin'>Welcome!<td></td>Enter your credentials to Sign Up</p>
                    <div className="dot">
                        <img src={miku} alt="Descriptive text" />
                    </div>
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
                    <div className="input-group">
                        <label className="label">Email</label>
                        <input autoComplete="off" name="info" id="info" className="input" placeholder='Enter your Email' type="email" />
                        <div></div>
                    </div>

                    <div className="radio-buttons-container">
                     <div className="radio-button">
                      <input name="radio-group" id="radio2" className="radio-button__input" type="radio" />
                       <label htmlFor="radio2" className="radio-button__label">
                         <span className="radio-button__custom"></span>
                         I agree to something
                                   </label>
                           </div>
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
