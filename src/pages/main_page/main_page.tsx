import './main_page.css';
import { DndProvider } from 'react-dnd';
import { HTML5Backend } from 'react-dnd-html5-backend';
import { useDrag } from 'react-dnd';
import ReactGridLayout from 'react-grid-layout';
import { useDrop } from 'react-dnd';

import './main_page.css';
import miku2 from '../../assets/25694.png'
import miku3 from '../../assets/search-icon.svg'


export default function main_page(){
    return <div id='background'>

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