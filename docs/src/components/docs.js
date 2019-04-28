import React from "react";
import {Link} from "gatsby";
import {MDXProvider} from '@mdx-js/react'

import Layout from "../components/layout"
import SEO from "../components/seo"
import './docs.css'

export default ({children, location}) => (
  <MDXProvider>
    <Layout>
        <SEO title="Stretch" keywords={[`rust`, `android`, `ios`, `web`, `flexbox`]} />
        <section>
          <div className="section-inner">
            <h2>Documentation</h2>
            <p>
              Stretch is available for Rust, Kotlin, Swift, and JavaScript. We will be adding more language bindings over time.
            </p>
            <div className="doc-pills">
              <Link to="/docs/rust" className={`doc-pill ${location.pathname.endsWith("rust") ? "active" : ""}`}>Rust</Link>
              <Link to="/docs/kotlin" className={`doc-pill ${location.pathname.endsWith("kotlin") ? "active" : ""}`}>Kotlin</Link>
              <Link to="/docs/swift" className={`doc-pill ${location.pathname.endsWith("swift") ? "active" : ""}`}>Swift</Link>
              <Link to="/docs/javascript" className={`doc-pill ${location.pathname.endsWith("javascript") ? "active" : ""}`}>JavaScript</Link>
            </div>
          </div>
        </section>
        <section className="mdx-content">
          <div className="section-inner">
            {children}
          </div>
        </section>
      </Layout>
  </MDXProvider>
);