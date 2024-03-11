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
}

function App({ element, name }: AppProps) {
    let [ref, dx, dy] = useDraggable({ gridSize: 10 });
                //@ts-expect-error
    const [appState, setAppState] = useState(State.Normal);
    dx = 500;
    dy = 200;
    return (
        <div 
            className="frame"
            style={{
                transform: `translate3d(${dx}px, ${dy}px, 0)`,
            }}
            //@ts-expect-error
            ref={ref}
            key={name}
        >
            <div className="frame-content">
                <h1 className="sexy">{name}</h1>
                <div className="frame-buttons">
                //@ts-expect-error
                    <button className="minimize-btn">-</button>
                    <button className="close-btn">x</button>
                </div>
            </div>
            
            <div className="palace"></div>
            {element}
            <h1>e</h1>
        </div>
        
    );
}

export default App;
