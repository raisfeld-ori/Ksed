            //@ts-expect-error

import React, { useState, RefObject } from "react";
import { useDraggable } from '../Grid';
import './App.css';

enum State {
    Normal,
    Minimized,
    FullScreen
}

interface AppProps {
    element: React.ReactElement;
    name: string;
    id: string;
}

function App({ element, name, id }: AppProps) {
    let [ref, dx, dy] = useDraggable({ gridSize: 10 });
    const [visibility, set_visibility] = useState('inherit');

    //@ts-expect-error
    const [appState, setAppState] = useState(State.Normal);

    return (
        <div 
            className="frame"
            style={{
                transform: `translate3d(${dx}px, ${dy}px, 0)`,
                display: `${visibility}`,
            }}
            id={id}
            //@ts-expect-error
            ref={ref}
            key={name}
        >
            <div className="frametest">
                <h1 className="sexy">{name}</h1>
                <div className="frame-buttons">
                    <button className="minimize-btn">-</button>
                    <button className="close-btn" onClick={()=>document.getElementById(id)?.remove()}>x</button>
                </div>
            </div>
            {element}
        </div>
        
    );
}

export default App;
