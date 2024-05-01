import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body>
      <link rel="stylesheet" href="global.css" />
      <nav>
      <div className="wrapper">
        <div className="logo"><a href="#">Logo</a></div>
        <input type="radio" name="slider" id="menu-btn" />
        <input type="radio" name="slider" id="close-btn" />
        <ul className="nav-links">
          <label htmlFor="close-btn" className="btn close-btn"><i className="fas fa-times"></i></label>
          <li><a href="/">Home</a></li>
          <li><a href="/about">About</a></li>
          <li>
            <a href="#" className="desktop-item">Purchases</a>
            <input type="checkbox" id="showDrop" />
            <label htmlFor="showDrop" className="mobile-item">Dropdown Menu</label>
            <ul className="drop-menu">
              <li><a href="#">Weekly Package</a></li>
              <li><a href="#">Monthly Package</a></li>
              <li><a href="#">Yearly Package</a></li>
              <li><a href="#">Lifetime Package</a></li>
            </ul>
          </li>
          <li><a href="#">Contact Us</a></li>
        </ul>
        <label htmlFor="menu-btn" className="btn menu-btn"><i className="fas fa-bars"></i></label>
      </div>
    </nav>
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function App() {
  return <Outlet />;
}
