import React from "react"
import Header from "./header"
import Footer from "./footer"
import "./layout.css"

export default function({ children }) {
  return (
    <>
      <Header />
        <main>
          {children}
        </main>
      <Footer />
    </>
  );
}
