import './main_page.css';
import './main_page.css';
import { useDraggable } from './Grid';
import miku2 from '../../assets/25694.png';
import miku3 from '../../assets/search-icon.svg';

export default function main_page(){
    const [draggableRef, dx, dy] = useDraggable({
        gridSize: 40,
    });

    const setDraggableRef = (node: HTMLDivElement | null) => {
        if (node !== null) {
            // idk how to fix this, but as long as it doesn't work, the unit testing wont run...
            draggableRef(node);
        }
    };

    return <div id='background'>

            <div className="container">
            <div
                className="draggable"
                ref={draggableRef}
                style={{
                    transform: `translate3d(${dx}px, ${dy}px, 0)`,
                }}
            >
                Drag me
            </div>
        </div>

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