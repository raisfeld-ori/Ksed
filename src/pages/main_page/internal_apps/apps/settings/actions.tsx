

export default function CategoryPage(props: {category: string}){
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
            <button>nothing here yet):</button>
        </div>
    }
}