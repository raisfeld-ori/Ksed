

export default function CategoryPage(props: {category: string}){
    switch (props.category){
        case "Welcome": return <div className="main-welcome">
            <h1>Welcome!</h1>
            <p>
                this is the welcome page of ksed <br />
            </p>
        </div>
        case "About": return <div>
            <h1>About Ksed</h1>
            <p>something something lorem ipsum</p>
        </div>
        default: return <div>error: unknown category</div>
    }
}