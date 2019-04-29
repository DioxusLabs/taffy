import React from "react";
import {Link} from "gatsby";
import {MDXProvider} from '@mdx-js/react'

import Layout from "../components/layout"
import SEO from "../components/seo"
import './docs.css'

import SyntaxHighlighter from 'react-syntax-highlighter';
import { atomOneDark } from 'react-syntax-highlighter/dist/esm/styles/hljs';

export default ({children, location}) => {
  const selected = (path) => {
    return location.pathname.endsWith(path) || location.pathname.endsWith(path + "/");
  }

  return (
    <MDXProvider>
      <Layout>
          <SEO title="Stretch" keywords={[`rust`, `android`, `ios`, `web`, `flexbox`]} />
          <section className="docs-header">
            <div className="section-inner">
              <h1>Documentation</h1>
              <p>
                Stretch is available for Rust, Kotlin, Swift, and JavaScript. We will be adding more language bindings over time.
                For general help on using flexbox we suggest 
                reading <a href="https://css-tricks.com/snippets/css/a-guide-to-flexbox/">css-tricks.com/snippets/css/a-guide-to-flexbox</a> and 
                playing around with <a href="https://flexboxfroggy.com">flexboxfroggy.com</a>. Or you can always read the spec itself
                at <a href="https://www.w3.org/TR/css-flexbox-1/">www.w3.org/TR/css-flexbox-1</a>
              </p>
              <div className="doc-pills">
                <Link to="/docs/rust" className={`doc-pill ${selected("rust") ? "active" : ""}`}>Rust</Link>
                <Link to="/docs/kotlin" className={`doc-pill ${selected("kotlin") ? "active" : ""}`}>Kotlin</Link>
                <Link to="/docs/swift" className={`doc-pill ${selected("swift") ? "active" : ""}`}>Swift</Link>
                <Link to="/docs/javascript" className={`doc-pill ${selected("javascript") ? "active" : ""}`}>JavaScript</Link>
              </div>
            </div>
          </section>
          <section className="mdx-content">
            <div className="section-inner">
              <div>
                {children}
              </div>
            </div>
          </section>
        </Layout>
    </MDXProvider>
  );
}

export function Code({file, lang, children}) {
  return (
      <div className="code-block">
          { file && <code>{file}</code> }
          <div className="code-wrapper">
            <SyntaxHighlighter language={lang} style={atomOneDark}>{children.trim()}</SyntaxHighlighter>
          </div>
      </div>
  );
}