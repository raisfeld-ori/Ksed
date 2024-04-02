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
        <h1 className="text"><span className="K">K</span><span className="sed">sed</span></h1>
        <h2 className="text">your private, hidden desktop</h2>
        <button>learn more</button>
      </div>
      <div className="square">
        <h1>new</h1>
      </div>
    </div>
  );
}
