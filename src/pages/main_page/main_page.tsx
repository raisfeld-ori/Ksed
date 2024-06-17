import { useEffect, useState } from 'react';
import './main_page.css';
import Grid from './Grid';
import folder from './assets/folder.png';
import settings from './assets/setting-icon.svg';
import menu_icon from './assets/computer-laptop-color-icon.webp';
import file_system from './internal_apps/apps/file_system/file_system';
import ibetonhakari from './assets/TOCA2.mp4';
import { desktop_app } from './Grid';
import leaveicon from './assets/leave.png';
import { useLocation, useNavigate } from 'react-router-dom';
import exit from './assets/exit.png';
import { invoke } from '@tauri-apps/api';
import Settings from './internal_apps/apps/settings/settings';
import sudo from './internal_apps/apps/sudo/sudo';
import text_viewer from './internal_apps/apps/text_viewer/text_viewer';
import text_icon from './assets/pencil-square-icon.svg';
import image_viewer from './internal_apps/apps/image_viewer/image_viewer';
import image_icon from './assets/image-icon.svg';
import html_viewer from './internal_apps/apps/html_viewer/html_viewer';
import html_icon from './assets/web-page-source-code-icon.svg';
import video_viewer from './internal_apps/apps/video_viewer/vide_viewer';
import video_icon from './assets/camera-roll-icon.svg';
import NotificationSystem from './internal_apps/notification';

function BinIcon(props: {display: () => Promise<void>, name: string, img: string}){
    return   <div className='appsmenu'onClick={props.display}>
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

const Clock = () => {
    const [currentTime, setCurrentTime] = useState(new Date());
  
    useEffect(() => {
      const interval = setInterval(() => {
        setCurrentTime(new Date());
      }, 1000);
  
      return () => clearInterval(interval);
    }, []);

    var day = String(currentTime.getDate()).padStart(2, '0');
    var month = String(currentTime.getMonth() + 1).padStart(2, '0'); //January is 0!
    var year = currentTime.getFullYear();
  
    return (
        <p className='time'>{currentTime.toLocaleTimeString()}<br />{day + '/' + month + '/' + year}</p>
    );
  };

export default function MainPage() {
    const navigate = useNavigate();
    const notifications = NotificationSystem();
    const [file_selected, set_file_selected] = useState<string | null>(null);
    const sudo_props = sudo('You have been logged out, Please log in');
    const sudo_app = desktop_app('sudo', '', sudo_props, false);
    const text_viewer_props = text_viewer(file_selected);
    const text_viewer_app = desktop_app('text viewer', '', text_viewer_props, false);
    const image_viewer_props = image_viewer(file_selected);
    const image_viewer_app = desktop_app('image viewer', '', image_viewer_props, false);
    const html_viewer_props = html_viewer(file_selected);
    const html_viewer_app = desktop_app('html viewer', '', html_viewer_props, false);
    const video_viewer_props = video_viewer(file_selected);
    const video_viewer_app = desktop_app('video viewer', '', video_viewer_props, false);
    const settings_props = Settings();
    const settings_app = desktop_app("settings", settings, settings_props, true);
    useEffect(() => {
        let update = async () => {
            if (file_selected == null) {return;}
            let file_extension: string = await invoke('gather_type', {file: file_selected});
            console.log(file_extension);
            switch (file_extension){
                case "Text": {
                    await text_viewer_props.update();
                    return;
                }
                case "Image": {
                    await image_viewer_props.update();
                    return;
                }
                case "HTML": {
                    await html_viewer_props.update();
                    return;
                }
                default: {return;}
            }
        }
        update()
    }, [file_selected])
    let open_file = async (file: string) => {
        set_file_selected(file);
        let file_extension: string = await invoke('gather_type', {file});
        switch (file_extension){
            case "Text": {
                text_viewer_props.set_display('inherit');
                return;
            }
            case "Image": {
                image_viewer_props.set_display('inherit');
                return;
            }
            case "HTML": {
                html_viewer_props.set_display('inherit');
                return;
            }
            case "Video": {
                video_viewer_props.set_display('inherit');
                return;
            }
            default: {return;}
        }
    }
    const fs_props = file_system(open_file, notifications.new_notification);
    const explorer_app = desktop_app("Files", folder, fs_props, true);

    const [menu, set_menu] = useState(false);
    useEffect(() => {
        fs_props.update();
    }, [useLocation().state]);
    useEffect(() => {
        const interval = setInterval(async () => {
            let name = await invoke('system_get', {key: 'name'});
            let password = await invoke('system_get', {key: 'password'});
            if (name == null || password == null) {sudo_props.set_display('inherit');sudo_props.fullscreen(true);return}
            const authenticated = await invoke('authenticate_user', {name, password});
            if (!authenticated) {sudo_props.set_display('inherit');sudo_props.fullscreen(true);return}
            else {sudo_props.set_display('none');sudo_props.fullscreen(false);}
        }, 0);
        return () => clearInterval(interval);
      }, []);
    return (
        <div id='background' onContextMenu={e => {e.preventDefault();}} onClick={() => {if (menu) {set_menu(false)}}}>
            {notifications.html}
            <Grid  apps={[explorer_app, settings_app, sudo_app, text_viewer_app, image_viewer_app,
    html_viewer_app, video_viewer_app,
            ]} gridSize={50} margin={120} />
            <nav className='navbar' onContextMenu={e => e.preventDefault()}>
                <img className='homeimg' onClick={() => set_menu(!menu)} src={menu_icon} alt="" />
                <Clock></Clock>
            </nav>
            <div className={`menu ${menu ? 'show' : 'hide'}`}>
            <div className="menu-header">
            <h1 className='menutext'>Applications</h1>


                <BinIcon display={async () => {fs_props.update();fs_props.set_display('inherit');}} 
                name='Files' img={folder} />
                <BinIcon display={async () => {text_viewer_props.set_display('inherit')}}
                name='text viewer' img={text_icon}/>
                <BinIcon display={async () => {html_viewer_props.set_display('inherit')}}
                name='html viewer' img={html_icon}/>
                <BinIcon display={async () => {video_viewer_props.set_display('inherit')}}
                name='video viewer' img={video_icon}/>
                <BinIcon display={async () => {image_viewer_props.set_display('inherit')}}
                name='image viewer' img={image_icon}/>

                <div className='menubutton'>
                         <p className='hiddenclose'>Exit 😭​</p>
                <button className='exit' onClick={async () => {await save_user();await invoke('close_app', {});}}>
                    <img src={exit} alt="exiticon" />
                </button>

                <p className='hiddentxt'>Logout</p>
                <button className='leave'onClick={async () => {await save_user();navigate("/", {state: {name: '', password: '', refresh: false}});}} >
                    <img src={leaveicon} alt="leaveicon" />
                </button>
                </div>
       
                </div>
                </div>


            <video className='hakari' src={ibetonhakari} width="100%" height="100%" autoPlay muted loop>
    Your browser does not support the video tag
</video>
        </div>
    );
}