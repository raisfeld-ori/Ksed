            //@ts-expect-error

import React, { useState, RefObject } from "react";
import { useDraggable } from '../Grid';
import './App.css';

enum State {
    Normal,
    Minimized,
    FullScreen,
    Closed,
}

interface AppProps {
    element: React.ReactElement;
    name: string;
}

function App({ element, name }: AppProps) {
    let [ref, dx, dy] = useDraggable({ gridSize: 10 });

    //@ts-expect-error
    const [appState, set_state] = useState(State.Normal);
    const [visibility, set_visibility] = useState('inherit');
    return (
        <div 
            className="frame"
            style={{
                transform: `translate3d(${dx}px, ${dy}px, 0)`,
                display: `${visibility}`,
            }}
            //@ts-expect-error
            ref={ref}
            key={name}
        >
            <div className="frametest">
                <h1 className="sexy">{name}</h1>
                <div className="frame-buttons">
                    <button className="minimize-btn">-</button>
                    <button className="close-btn" onClick={()=>{set_visibility('none');set_state(State.Closed);}}>x</button>
                </div>
            </div>
            {element}
        </div>
        
    );
}

export default App;
