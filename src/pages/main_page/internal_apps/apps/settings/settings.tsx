import App, { AppInterface } from "../../App";
import welcome from './assets/house-icon.svg';
import volume from './assets/megaphone-icon.svg';
import messages from './assets/bell-silent-line-icon.svg';
import about from './assets/info-circle-icon.svg';
import background from './assets/image-icon.svg';
import encryptions from './assets/lock.png';
import uploaded from './assets/Folder-1.png';

import './settings.css';
import { useState } from "react";
import CategoryPage from "./actions";

function CategoryHead(props: {title: string}){
    return <div className="category-head">
        <h1 className="title">{props.title}</h1>
    </div>
}

function Category(props: {name: string, icon: string, set_page: React.Dispatch<React.SetStateAction<JSX.Element>>, unique: string | undefined}){
    let clicked = () => {
        props.set_page(<CategoryPage category={props.name}/>);
    }
    return <div className={"category " + props.unique} onClick={clicked}>
        <img className="icon" src={props.icon}/>
        <h1 className="name">{props.name}</h1>
        <h1 className="end">{'>'}</h1>
    </div>
}

export default function Settings() : AppInterface{
    const [page, set_page] = useState(<CategoryPage category={'Welcome'}/>);
    let app_html = <div className="outer">
        <div className="categories">
            <CategoryHead  title="General"></CategoryHead>
            <Category unique="welcome" name="Welcome" icon={welcome} set_page={set_page}></Category>
            <Category unique="bout" name="About" icon={about} set_page={set_page}></Category>
            <CategoryHead title="Workspace"></CategoryHead>
            <Category unique="background2" name="Background" icon={background} set_page={set_page}></Category>
            <Category unique="volume" name="Volume" icon={volume} set_page={set_page}></Category>
            <Category unique="encrypt" name="Encryptions" icon={encryptions} set_page={set_page}></Category>
            <Category unique="uploaded" name="Uploaded" icon={uploaded} set_page={set_page}></Category>
            <Category unique="messages" name="Notifications" icon={messages} set_page={set_page}></Category>

        </div>
        <div className="category-inner">
            {page}
        </div>
    </div>
    let context_menu = <h1></h1>
    let update = async () => {}
    let [screen, set_display, fullscreen] = App(app_html, "settings", true);
    return {screen, set_display, context_menu, update, fullscreen};
}

