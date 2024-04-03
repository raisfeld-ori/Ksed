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
}

function App(element: JSX.Element, name: string): [JSX.Element, React.Dispatch<React.SetStateAction<string>>] {
    const [display, set_display] = useState('inherit');
    let [ref, dx, dy] = useDraggable({ gridSize: 10 });
    return [
        <div 
            className="frame"
            style={{
                transform: `translate3d(${dx}px, ${dy}px, 0)`,
                display: display,
            }}
            //@ts-expect-error
            ref={ref}
            key={name}
        >
            <div className="frametest">
                <h1 className="frametext2">{name}</h1>
                <div className="frame-buttons">
                    <button className="minimize-btn">-</button>
                    <button className="close-btn" onClick={()=>{set_display('none');}}>x</button>
                </div>
            </div>
            {element}
        </div>
        , set_display];
}

export default App;