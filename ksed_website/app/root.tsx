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

  	<input className="menu-icon" type="checkbox" id="menu-icon" name="menu-icon"/>
    <label htmlFor="menu-icon"></label>
  	<nav className="nav">
  		<ul className="pt-5">
  			<li><a href="#">get back to Work!</a></li>
  			<li><a href="#">Studio</a></li>
  			<li><a href="#">News</a></li>
  			<li><a href="#">Contact</a></li>
  		</ul>
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
