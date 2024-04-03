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
  			<li><a href="/buy">buy the app</a></li>
  			<li><a href="/plans">future plans</a></li>
  			<li><a href="/support">support us</a></li>
  			<li><a href="/contact">Contact us</a></li>
        <li><a href="/">main page</a></li>
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
