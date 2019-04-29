import React from "react"
import { Link } from "gatsby"
import logo from "../images/logo.svg"
import "./header.css"

export default function Header() {
  return (
    <header>
      <a href="https://visly.app" className="logo">
        <img src={logo}/>
      </a>
      <h3>Visly</h3>
      <div className="spacer"/>
      <a href="https://www.twitter.com/vislyhq" className="nav">
          Twitter
      </a>
      <a href="https://www.github.com/vislyhq/stretch" className="nav">
          Github
      </a>
      <a href="https://www.medium.com/visly" className="nav">
        Blog
      </a>
      <Link to="/docs/rust" className="nav">
        Docs
      </Link>
    </header>
  );
}
