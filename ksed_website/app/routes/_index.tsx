import type { MetaFunction } from "@remix-run/node";
import './style/index.css';
export const meta: MetaFunction = () => {
  return [
    { title: "Ksed" },
    { name: "Ksed", content: "your private hidden desktop" },
  ];
};

export default function Index() {
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
          <h1 className="text">its justice!</h1>
          <p className="text">
            Lorem ipsum dolor, sit amet 
            consectetur adipisicing elit. 
            Earum voluptatem esse minima, 
            quisquam, sed dolore iste id 
            repudiandae corporis saepe, 
            explicabo ullam? Assumenda 
            ullam corrupti eius ipsum 
            distinctio corporis nostrum.
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
        <button className="navbutton">Slaves</button>
        <button className="navbutton">pls money</button>
        <button className="navbutton">click for cookies</button>
        <button className="navbutton">Women Rights</button>
        </div>
        <div className="row">
          <div className="container">
            <h1 className="sqrtextleftheadhead">Your lie in april</h1>
            <p className="sqrtextleft">Kimi da yo  kimi nanda yo  egao wo kureta
               Namida mo hikaru nara  ryuusei ni naru
                Kizutsuita sono te wo  mou hanasanai de
            </p>
            <div className="linesqr"></div>
            <h1 className="sqrtextlefthead">Your lie in april</h1>
            <p className="sqrtextleft">Kimi da yo  kimi nanda yo  egao wo kureta
               Namida mo hikaru nara  ryuusei ni naru
                Kizutsuita sono te wo  mou hanasanai de
            </p>
          </div>
          <div className="container">
            <h1 className="textinsquareheaderheader">one fool thought he'd find, hoo.</h1>
            <p className="textinsquare">
            Purpose in his life along the way
            Don't you run and hide from the truth, you decide
            Everything that lives is gone to waste
            </p>
            <div className="linesqr2"></div>

            <h1 className="textinsquare">Don't you run and hide from the truth</h1>
            <p className="textinsquare">
            you decide Everything that lives is gone to waste
            Once upon a time, was a fool who thought he'd find
            Purpose in his life along the way
            </p>
            <div className="squareinsquareinsquare"></div>
          </div>
        </div>
      </div>
      </div>
    </div>
  );
}
