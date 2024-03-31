import { Html, Head, Main, NextScript } from 'next/document';
export default function Document() {
  return (
    <Html lang="en">
      <link rel="stylesheet" href="globals.css" />
      <Head />
      <body>
      <nav>
      <div className="wrapper">
        <div className="logo"><a href="#">
          <img src="logo.png" alt="" />
          </a></div>
        <input type="radio" name="slider" id="menu-btn" />
        <input type="radio" name="slider" id="close-btn" />
        <ul className="nav-links">
          <label htmlFor="close-btn" className="btn close-btn"><i className="fas fa-times"></i></label>
          <li><a href="#">Home</a></li>
          <li><a href="#">About</a></li>
          <li>
            <a href="#" className="desktop-item">Purchase</a>
            <input type="checkbox" id="showDrop" />
            <label htmlFor="showDrop" className="mobile-item">Purchase</label>
            <ul className="drop-menu">
              <li><a href="#">Weekly</a></li>
              <li><a href="#">Monthly</a></li>
              <li><a href="#">Yearly</a></li>
              <li><a href="#">Lifetime</a></li>
            </ul>
          </li>
          <li>
          </li>
          <li><a href="#">Feedback</a></li>
        </ul>
        <label htmlFor="menu-btn" className="btn menu-btn"><i className="fas fa-bars"></i></label>
      </div>
    </nav>

      <Main />
      <NextScript />
      </body>
    </Html>
  )
}