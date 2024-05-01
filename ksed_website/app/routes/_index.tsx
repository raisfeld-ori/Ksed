import type { MetaFunction } from "@remix-run/node";
import { gsap } from "gsap";
import './style/index.css';
import { useEffect, useState } from "react";
import React from 'react';
import { useNavigate } from "react-router";
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
  const popRef = React.createRef<HTMLDivElement>();
  const navigate = useNavigate();

 useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          gsap.set('.left', {
            x: '0%',
            rotate: 0,
          });
          gsap.set('.right', {
            x: '0%',
            rotate: 0,
          });
          if (entry.isIntersecting) {
            entry.target.classList.add('visible');
            entry.target.classList.remove('invisible');
          }
          else{
            entry.target.classList.remove('visible');
            entry.target.classList.add('invisible');
            gsap.set('.left', {
              x: '20%',
              rotate: 10,
              duration: 0.85,
              scrollTrigger: {
                trigger: '.pop',
                scrub: true,
                start: 'top 20%',
              }
           });
           gsap.set('.right', {
            x: '-20%',
            rotate: 10,
            duration: 0.85,
            scrollTrigger: {
              trigger: '.pop',
              scrub: true,
              start: 'top 20%',
            }
         });
          }
        });
      },
    );
    if (popRef.current) {
      const popElements = popRef.current.querySelectorAll('.pop');
      popElements.forEach((element: Element) => {
        observer.observe(element);
      });
    }
    return () => {
      if (popRef.current) {
        const popElements = popRef.current.querySelectorAll('.pop');
        popElements.forEach((element: Element) => {
          observer.unobserve(element);
        });
      }
    };
 }, []);
  const [selected, set_selected] = useState("A");
  return (
    <div className="main" ref={popRef}>
      <div className="square">
        <div className="column">
          <h1 className="text"><span id="K">K</span><span id="sed">sed</span></h1>
          <h2 className="textpmain">Your hidden desktop</h2>
          <button className="learn" onClick={() => navigate('about')}>learn more</button>
        </div>
      </div>
      <div className="column" style={{justifyContent: "unset"}}>
      <div className="exception"><h1 id="whatis" className="pop">What is Ksed?</h1></div>
       <div id="text0"className="ilegaltext left">Secure</div>
        <div id="text1"className="ilegaltext right">Beautiful</div>
        <div id="text2"className="ilegaltext left">Easy to use</div>
        <div id="text3"className="ilegaltext right">Safe</div>
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

            <h1 className="textinsquareheader">{get_text(selected)[3].title}</h1>
            <p className="textinsquare">
            {get_text(selected)[3].text}
            </p>
            <div className="linesqr2"></div>
            <div className="squareinsquareinsquare"></div>
            
          </div>
        </div>
      </div>
      </div>
    </div>
  );
}
