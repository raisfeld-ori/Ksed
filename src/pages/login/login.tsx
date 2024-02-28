import './login.css';
import { window } from '@tauri-apps/api';
import { useState } from 'react';

function login(){
    return <div id='background'>
        <button onClick={() => window.appWindow.close()} style={{'position': "absolute"}}>
            X
        </button>
        <div className='triangle down'></div>
        <div className='triangle left'></div>
        <div id='menu'>
            <br />
            <button>login</button>
            <button>sign up</button>

            <form>
                <button>log in</button>
            </form>
        </div>
    </div>
}

export default login;