            //@ts-expect-error

import React, { useState, RefObject } from "react";
import { useDraggable } from '../Grid';
import './App.css';

export enum State {
    Normal,
    Minimized,
    FullScreen,
    Closed,
}

interface AppProps {
    element: React.ReactElement;
    name: string;
    display: string,
    set_display:  React.Dispatch<React.SetStateAction<string>>,
}

function App({ element, name, display, set_display }: AppProps) {
    let [ref, dx, dy] = useDraggable({ gridSize: 10 });
    return [
        <div 
            className="frame"
            style={{
                transform: `translate3d(${dx}px, ${dy}px, 0)`,
                display: `${display}`,
            }}
            //@ts-expect-error
            ref={ref}
            key={name}
        >
            <div className="frametest">
                <h1 className="sexy">{name}</h1>
                <div className="frame-buttons">
                    <button className="minimize-btn">-</button>
                    <button className="close-btn" onClick={()=>{set_display('none');}}>x</button>
                </div>
            </div>
            {element}
        </div>
        ];
}

export default App;
