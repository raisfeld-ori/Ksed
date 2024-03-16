import { useEffect, useState } from 'react';
import './main_page.css';
import Grid from './Grid';
import folder from './assets/folder.png';
import menu_icon from './assets/computer-laptop-color-icon.webp';
<<<<<<< HEAD
import search from './assets/search.png';
import terminald from './assets/terminal.png';
=======
import search from './assets/search-icon.svg';
import terminald from './assets/Terminal-icon.png';
>>>>>>> c1541b44c7395eafd8f1fdc951aeb5c6ab2dbdeb
import file_system from './internal_apps/file_system';
import ibetonhakari from './assets/TOCA2.mp4';;
import { desktop_app } from './Grid';


export default function MainPage() {
<<<<<<< HEAD
    const [fs, set_fs] = useState(<></>);
    const example_app = desktop_app("Files", folder, file_system(set_fs));
=======
    const [app, fs_display] = file_system();
    const example_app = desktop_app("Dolphin", folder, () => {fs_display('inherit')});
>>>>>>> c1541b44c7395eafd8f1fdc951aeb5c6ab2dbdeb
    const not_example_app = desktop_app("Search", search, () => { console.log("test"); });
    const terminal = desktop_app("Terminal", terminald, () => {});
    const [menu, set_menu] = useState(false);
    return (
        <div id='background' onContextMenu={e => {e.preventDefault();}}>
            {app}
            <Grid  apps={[example_app, not_example_app, terminal]} gridSize={50} margin={120} />
            <nav className='navbar' onContextMenu={e => e.preventDefault()}>
                <img className='homeimg' onClick={() => set_menu(!menu)} src={menu_icon} alt="" />
                <img className='searchimg' src={search} alt="" />
                <img className='terminalimg' src={terminald} alt="" />
                <p className='time'>12:09 AM <br /> 12/10/2027</p>
            </nav>
<<<<<<< HEAD
            <video className='hakari' src={ibetonhakari} width="100%" height="100%" autoPlay loop muted>
                Your browser does not support the video tag.
            </video>
=======
            <div className={`menu ${menu ? 'show' : 'hide'}`}>
                <h1>test</h1>
            </div>
            <video className='hakari' src={ibetonhakari} width="100%" height="100%" autoPlay muted loop>
    Your browser does not support the video tag.
</video>
>>>>>>> c1541b44c7395eafd8f1fdc951aeb5c6ab2dbdeb
        </div>
    );
}