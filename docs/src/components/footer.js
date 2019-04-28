import React from "react"
import logo from "../images/logo.svg"
import "./footer.css"

export default function Footer() {
  return (
      <footer>
          <div className="footer-inner">
              <a href="https://visly.app" className="logo">
                <img src={logo}/>
              </a>
              <div className="footer-sections">
                  <div className="footer-links">
                      <h3>Visly</h3>
                      <a href="https://www.twitter.com/vislyhq">Twitter</a>
                      <a href="https://www.github.com/vislyhq">Github</a>
                      <a href="https://www.medium.com/visly">Blog</a>
                  </div>
                  <div className="footer-links">
                      <h3>Company</h3>
                      <a href="mailto:careers@visly.app">Careers</a>
                      <a href="mailto:hello@visly.app">Contact</a>
                  </div>
              </div>
          </div>
      </footer>
  )
}