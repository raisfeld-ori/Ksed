import React, {useState} from "react";

enum state{
    Normal,
    Minimized,
    FullScreen
}

function App(props: {element: React.JSX.Element}){
    const [app_state, set_app_state] = useState(state.Normal)
    return <h1>this is the frame, {props.element}</h1>
}

export default App;