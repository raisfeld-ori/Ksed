import './main_page.css';
import './main_page.css';
import { desktop_app } from './Grid';
import Grid from './Grid';

import miku2 from '../../assets/25694.png';
import miku3 from '../../assets/search-icon.svg';

export default function main_page(){
    const example_app = desktop_app("example app", miku3);
    const not_example_app = desktop_app("not example app", miku3);

    return <div id='background'>
        <Grid apps={[example_app, not_example_app]} gridSize={50} margin={100}/>
        <nav className='navbar'>
        <button>
            <img className='homeimg' src={miku2} alt="" />
        </button>
            <button>
                <img className='searchimg' src={miku3} alt="" />
            </button>
         <p className='time'>12:09 AM</p>
         <p className='time'>12/10/2027</p>
        </nav>
    </div>
}