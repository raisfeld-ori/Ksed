import React, {useState} from "react";
import {useDraggable} from '../Grid';
import './App.css';

enum state{
    Normal,
    Minimized,
    FullScreen
}

function App(props: {element: React.JSX.Element, name: string}){
    const [ref, dx, dy] = useDraggable({gridSize: 10});
    const [app_state, set_app_state] = useState(state.Normal)
    return <div 
    className="frame"
    style={{
        transform: `translate3d(${dx}px, ${dy}px, 0)`,
          }}
    //@ts-expect-error
    ref={ref}
    key={props.name}
    >
        {props.element}
        </div>
}

export default App;