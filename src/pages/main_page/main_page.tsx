
import React, { useState } from 'react';
import './main_page.css';

import Grid from './Grid';
import folder from './assets/folder.png';
import menu_icon from './assets/computer-laptop-color-icon.webp';

import terminald from './assets/Terminal-icon.png';
import file_system from './internal_apps/file_system';
import ibetonhakari from './assets/TOCA2.mp4';;
import { desktop_app } from './Grid';


export default function MainPage() {
    const [fs, set_fs] = useState(<></>);
    const example_app = desktop_app("Dolphin", folder, file_system(set_fs));
    const not_example_app = desktop_app("Search", search, () => { console.log("test"); });
    const terminal = desktop_app("Terminal", terminald, () => {});

    return (
        <div id='background'>

            {fs}
            <Grid apps={[example_app, not_example_app, terminal]} gridSize={50} margin={120} />
            <nav className='navbar'>
                <img className='homeimg' src={menu_icon} alt="" />
                <img className='searchimg' src={search} alt="" />
                <img className='terminalimg' src={terminald} alt="" />
                <p className='time'>12:09 AM <br /> 12/10/2027</p>
            </nav>
            <video className='hakari' src={ibetonhakari} width="100%" height="100%" autoPlay muted>
    Your browser does not support the video tag.
</video>
        </div>
    );
}