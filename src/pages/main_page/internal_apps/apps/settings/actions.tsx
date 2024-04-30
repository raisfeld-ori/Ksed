

export default function CategoryPage(props: {category: string}){
    switch (props.category){
        case "Welcome": return <div>
            <h1>Welcome!</h1>
            <p>
                this is the welcome page of ksed <br />
                Lorem ipsum dolor sit amet consectetur, adipisicing elit. Velit, assumenda fugit 
                perspiciatis debitis iste aliquid obcaecati. 
                Minus temporibus deleniti dolorem veniam adipisci? Modi veniam, 
                error mollitia aspernatur autem eum quidem!
            </p>
            <h1>something about Ksed</h1>
            <p>
            पढाए हिंदी रहारुप अनुवाद कार्यलय मुख्य संस्था सोफ़तवेर निरपेक्ष उनका आपके बाटते 
            आशाआपस मुख्यतह उशकी करता। 
            शुरुआत संस्था कुशलता मेंभटृ अनुवाद गएआप विशेष सकते 
            परिभाषित लाभान्वित प्रति देकर समजते दिशामे प्राप्त जैसे वर्णन संस्थान निर्माता प्रव्रुति भाति चुनने उपलब्ध बेंगलूर
            </p>
            <h1>join us!</h1>
            <p>
            Kraft meiner Seele ungenutzt? Konnt' ich dafür, daß, 
            während die eigensinnigen Reize 
            ihrer Schwester Sophie folgen, als wenn sie's selber wäre, 
            das sie nun fast so gern von mir als von mir". 
            Ich machte ihr ein unbedeutendes Kompliment, meine ganze Seele anzogen—wie ich, in den schwarzen Augen.
            </p>
        </div>
        default: return <div>error: unknown category</div>
    }
}