import React, { useState } from "react";
import { useDraggable } from '../Grid';
import './App.css';

export enum State {
    Normal,
    Minimized,
    FullScreen,
    Closed,
}
export interface AppInterface{
    screen: JSX.Element,
    context_menu: JSX.Element,
    update: () => Promise<void>,
    set_display: React.Dispatch<React.SetStateAction<string>>,
    fullscreen: (active: boolean) => void,
}


function App(element: JSX.Element, name: string, use_fullscreen?: boolean): [JSX.Element, React.Dispatch<React.SetStateAction<string>>, (active: boolean) => void] {
    const [display, set_display] = useState('inherit');
    let [ref, dx, dy] = useDraggable({ gridSize: 10 });
    const [is_fullscreen, set_fullscreen] = useState(false);
    const [screensize, set_screensize] = useState({width: `875px`, height: `50svh`});
    let fullscreen = (active: boolean) => {
        set_fullscreen(active)
        if (!active){
            set_screensize({width: `875px`, height: `50svh`});
        }
        else{
            set_screensize({width: `100svw`, height: `93svh`});
        }
        return is_fullscreen;
    };
    let position = () => {
        if (is_fullscreen){
            return `translate3d(0px, 0px, 0)`;
        }
        else{
            return `translate3d(${dx}px, ${dy}px, 0)`;
        }
    }
    return [
        <div 
            className="frame"
            style={{
                transform: position(),
                display: display,
            }}
            //@ts-expect-error
            ref={ref}
            key={name}
        >
            <div className="frametest" style={{width: screensize.width}}>
                <h1 className="frametext2">{name}</h1>
                    <button className="minimize-btn" onClick={() => set_display('none')}>-</button>
                    {use_fullscreen == undefined || use_fullscreen ? <button className="fullscreen-btn" onClick={() => fullscreen(!is_fullscreen)}>↔</button> : null}
                    <button className="close-btn" onClick={()=>{set_display('none');}}>x</button>
            </div>
            <div className="background" style={{width: screensize.width, height: screensize.height }}>
            {element}
            </div>
        </div>
        , set_display, fullscreen];
}

export default App;