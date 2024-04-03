import { useState } from 'react';
import './main_page.css';
import Grid from './Grid';
import folder from './assets/folder.png';
import menu_icon from './assets/computer-laptop-color-icon.webp';
import search from './assets/search.png';
import terminal from './assets/terminal.png';
import file_system from './internal_apps/apps/file_system';
import ibetonhakari from './assets/TOCA2.mp4';
import { desktop_app } from './Grid';
import leaveicon from './assets/leave.png';
import { useNavigate } from 'react-router-dom';
import exit from './assets/exit.png';
import { invoke } from '@tauri-apps/api';

function BinIcon(props: {display: () => Promise<void>, name: string, img: string}){
    return   <div className='appsmenu'onClick={() => {props.display}}>
    <p><i className="line right"></i></p>
    <button className='folderappmenu'>
        <img className='folderappmenu' src={props.img} alt="filesystem" />
    </button>
    <p className='filestxt'>{props.name}</p>
    </div>;
}

async function save_user(){
    let username = await invoke('system_get', {key: 'name'});
    let password = await invoke('system_get', {key: 'password'});
    await invoke('save_user', {username, password});
}

export default function MainPage() {
    const navigate = useNavigate();
    const {screen, context_menu, update, set_display} = file_system();
    const explorer_app = desktop_app("Files", folder, async () => {await update();set_display('inherit');});
    const search_apps = desktop_app("Search", search, async () => { set_menu(true);});
    const [menu, set_menu] = useState(false);
    return (
        <div id='background' onContextMenu={e => {e.preventDefault();}} onClick={() => {if (menu) {set_menu(false)}}}>
            {context_menu}
            {screen}
            <Grid  apps={[explorer_app, search_apps]} gridSize={50} margin={120} />
            <nav className='navbar' onContextMenu={e => e.preventDefault()}>
                <img className='homeimg' onClick={() => set_menu(!menu)} src={menu_icon} alt="" />
                <p className='time'>12:09 AM <br /> 12/10/2027</p>
            </nav>
            <div className={`menu ${menu ? 'show' : 'hide'}`}>
            <div className="menu-header">
            <h1 className='menutext'>Applications</h1>

            </div>
                <button className='leave'onClick={async () => {await save_user();navigate("/");}} >
                    <img src={leaveicon} alt="leaveicon" />
                </button>
                <p className='hiddentxt'>Logout</p>
                <button className='exit' onClick={async () => {await save_user();await invoke('close_app', {});}}>
                    <img src={exit} alt="exiticon" />
                </button>
                <p className='hiddenclose'>Exit 😭​</p>
                
                <BinIcon display={async () => {await update();set_display('inherit');}} name='Files' img={folder}></BinIcon>
                <BinIcon display={async () => {await update();set_display('inherit');}} name='Terminal' img={terminal}></BinIcon>

                
            </div>
            <video className='hakari' src={ibetonhakari} width="100%" height="100%" autoPlay muted loop>
    Your browser does not support the video tag
</video>
        </div>
    );
}