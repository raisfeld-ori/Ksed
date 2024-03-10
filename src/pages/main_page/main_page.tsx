import './main_page.css';
import './main_page.css';
import { desktop_app } from './Grid';
import Grid from './Grid';
import folder from './assets/folder-line-icon.svg';
import menu_icon from './assets/laptop-icon.svg';
import search from './assets/search-icon.svg';
import file_system from './internal_apps/file_system';
import { useState } from 'react';

export default function main_page(){
    const [fs, set_fs] = useState(<></>);
    const example_app = desktop_app("example app", folder, file_system(set_fs));
    const not_example_app = desktop_app("not example app", search, () => {console.log("test");});

    return <div id='background'>
        {fs}
        <Grid apps={[example_app, not_example_app]} gridSize={50} margin={100}/>
        <nav className='navbar'>
        <img className='homeimg' src={menu_icon} alt="" />
        <img className='searchimg' src={search} alt="" />
         <p className='time'>12:09 AM</p>
         <p className='time'>12/10/2027</p>
        </nav>
    </div>
}