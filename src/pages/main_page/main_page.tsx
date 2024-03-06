import './main_page.css';
import App from './make_app';

export default function main_page(){


    return <div id='background'>
        <div id='desktop'>
            <App />
        </div>
        <div id='taskbar'></div>
    </div>
}