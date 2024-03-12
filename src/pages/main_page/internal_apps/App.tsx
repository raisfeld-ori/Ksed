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
            <div className="frametest">
                <h1 className="sexy">{name}</h1>
                <div className="frame-buttons">
                    <button className="minimize-btn">-</button>
                    <button className="close-btn">x</button>
                </div>
            </div>
            {element}
        </div>
        
    );
}

export default App;
