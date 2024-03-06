import './main_page.css';
import miku2 from '../../assets/avfe23a6b11d1ff4752fd.png'

export default function main_page(){


    return <div id='background'>
        <div id='desktop'></div>
        <div id='taskbar'></div>

        <nav className='navbarmain'>
            <button>
                <img src={miku2} alt="" />
            </button>
        </nav>
    </div>
}