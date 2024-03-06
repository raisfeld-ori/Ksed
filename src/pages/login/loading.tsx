import './login.css';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
import { useLocation } from 'react-router-dom';
import {useEffect} from "react";


// loading the page requires a username and a password
export default function Loading(){
    const location = useLocation();
    useEffect(() => {
        if (!location.state) {
            return;
        }
        const loadUser = async () => {
            await invoke("load_user", location.state);
        };
        loadUser();
    }, [location.state]);
    if (!location.state){return <h1>DONT ACCESS THIS PAGE WITHOUT LOGGING IN</h1>}
    return <div id='background'>
    <div className='triangle down'></div>
        <div className='triangle left'></div>
        <div id='user'>
            <h1>welcome {location.state.name}!</h1>
            <progress max={10} value={1}></progress>
        </div>
    </div>
}