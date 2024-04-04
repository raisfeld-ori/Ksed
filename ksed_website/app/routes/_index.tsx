import type { MetaFunction } from "@remix-run/node";
import './style/index.css';
import { useState } from "react";
export const meta: MetaFunction = () => {
  return [
    { title: "Ksed" },
    { name: "Ksed", content: "your private hidden desktop" },
  ];
};

function is_selected(selected: string, name: string){
    if (selected == name){return 'selected'}
    else {return ''}
}
function get_text(selected: string): {title: string, text: string}[]{
  switch (selected){
    case "A":
      return [{title: "A", text: "A"}, {title: "A", text: "A"}, {title: "A", text: "A"}, {title: "A", text: "A"}];
    case "B":
      return [{title: "B", text: "B"}, {title: "B", text: "B"}, {title: "B", text: "B"}, {title: "B", text: "B"}];
    default:
      return [{title: "null", text: "null"}, {title: "null", text: "null"}, {title: "null", text: "null"}, {title: "null", text: "null"}];
  }
}

export default function Index() {
  const [selected, set_selected] = useState("A");
  return (
    <div className="main">
      <div className="square">
        <div className="column">
          <h1 className="text"><span id="K">K</span><span id="sed">sed</span></h1>
          <h2 className="textpmain">Your hidden desktop</h2>
          <button className="learn">learn more</button>
        </div>
      </div>
      <div className="square">
        <div className="exception"><h1>what is Ksed?</h1></div>
        <div className="row">
        <div className="container">
          <h1 className="text">it's your own, hidden desktop</h1>
          <p className="text">
            do you have important files? or something that you want to hide.
            it could be that you job requires you to have some important files,
            or maybe you just have a secret hobby that you want to hide.
            well, that's what ksed is for! it's a hidden app in your computer,
            that after being opened, will act like a desktop, except everything you
            do is hidden, and after finishing, the app will just save the state of
            the machine the last time you used it and then continue hiding like you
            didn't even use it.
          </p> 
          <h1 className="text">Ksed = desK</h1>
          <p className="text">
          応発ヨオ訪合に手41否ひみぱ毒京基をフ由禁はぎふ著5横ユヱ議読合じまん載批よーむリ就保ば浦図マソリ付近ヨ背調帳懸ら。
          転ゆでぴ菓南ーが統想モレアオ調日わレぞら極指べる並遺フレンき新組づべぽへ崎典ソフムキ購補写ラサオ販理正教わすえ雪球ナケ引自タニツ観候幹び。
          </p>
        </div>
        <div className="container">
          <img className="sized" src="alpha.jpeg"></img>
        </div>
        </div>
      </div>
      <div className="square">
      <div className="squareinsquare">
        <div className="exception"><h1>Questions ✦ Answers</h1></div>

      <div className="navbuttons">
        <button 
        className={"navbutton " + is_selected(selected, "A")}
        onClick={() => set_selected('A')}
        >A
        </button>
        <button 
        className={"navbutton " + is_selected(selected, "B")}
        onClick={() => set_selected('B')}
        >B
        </button>
        <button 
        className={"navbutton " + is_selected(selected, "C")}
        onClick={() => set_selected('C')}
        >C
        </button>
        <button 
        className={"navbutton " + is_selected(selected, "D")}
        onClick={() => set_selected('D')}
        >D
        </button>
        </div>
        <div className="row">
          <div className="container">
            <h1 className="sqrtextleftheadhead">{get_text(selected)[0].title}</h1>
            <p className="sqrtextleft">
              {get_text(selected)[0].text}
            </p>
            <div className="linesqr"></div>
            <h1 className="sqrtextleftheadhead">{get_text(selected)[1].title}</h1>
            <p className="sqrtextleft">
              {get_text(selected)[1].text}
            </p>
            <div className="linesqr"></div>
          </div>
          <div className="container">
            <h1 className="textinsquareheaderheader">{get_text(selected)[2].title}</h1>
            <p className="textinsquare">
            {get_text(selected)[2].text}
            </p>
            <div className="linesqr2"></div>

            <h1 className="textinsquare">{get_text(selected)[3].title}</h1>
            <p className="textinsquare">
            {get_text(selected)[3].text}
            </p>
            <div className="squareinsquareinsquare"></div>
          </div>
        </div>
      </div>
      </div>
    </div>
  );
}
