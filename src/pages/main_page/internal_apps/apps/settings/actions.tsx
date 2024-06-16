import { useState } from "react"


export default function CategoryPage(props: {category: string}){
    const [volume, set_volume] = useState(50);
    switch (props.category){
        case "Welcome": return <div className="settings-page main-welcome">
            <h1>Welcome!</h1>
            <p>
                this is the welcome page of ksed <br />
            </p>
        </div>
        case "About": return <div className="settings-page">
            <h1>About Ksed</h1>
            <p>something something lorem ipsum</p>
        </div>
        default: return <div>error: unknown category</div>
        case "Background": return <div className="settings-page">
            <h1>Set background</h1>
            <p>this will be added soon</p>
        </div>
        case "Volume": return <div className="settings-page">
            <h1>Set volume</h1>
            <p>{volume}</p>
            <input type="range" min={1} max={100} onChange={(e) => set_volume(e.target.valueAsNumber)}></input>
        </div>
        case "Encryptions": return <div className="settings-page">
            <button>nothing here yet...</button>
        </div>
        case "Uploaded": return <div className="settings-page">
            <button>nothing here yet...</button>
        </div>
        case "Notifications": return <div className="settings-page">
            <button>nothing here yet...</button>
        </div>
    }
}