import './login.css';
// import { useState } from 'react';
// import { useNavigate } from 'react-router-dom';
import { useLocation } from 'react-router-dom';

// loading the page requires a username and a password
function loading(){
    const location = useLocation();
    console.log(location.state)

    return <div id='background'>
        
    </div>
}

export default loading;