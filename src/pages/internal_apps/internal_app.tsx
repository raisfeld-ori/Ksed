import React, {Component} from "react";

enum state{
    Normal,
    Minimized,
    FullScreen
}

class Frame extends Component{
    element;
    state = state.Normal;

    constructor(props: {element: React.JSX.Element}){
        super(props);
        this.element = props.element;
    }
    render(){
        return <div className="frame"></div>
    }
}