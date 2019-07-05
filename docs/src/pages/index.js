import React from "react"
import {Link} from "gatsby"

import Layout from "../components/layout"
import SEO from "../components/seo"
import './index.css'

import logo from '../images/logo.svg';
import S from '../images/S.svg';
import T from '../images/T.svg';
import R from '../images/R.svg';
import E from '../images/E.svg';
import C from '../images/C.svg';
import H from '../images/H.svg';

import SyntaxHighlighter from 'react-syntax-highlighter';
import { tomorrow } from 'react-syntax-highlighter/dist/esm/styles/hljs';

const Code = ({file, lang, children}) => {
    return (
        <div className="code-block">
            <code>{file}</code>
            <SyntaxHighlighter language={lang} style={tomorrow}>{children}</SyntaxHighlighter>
        </div>
    );
}

const rustSnippet = `
use stretch::{style::*, node::*, geometry::Size};

let stretch = Stretch::new();

let node = stretch.new_node(Style {
    size: Size { 
        width: Dimension::Points(100.0), 
        height: Dimension::Points(100.0),
    },
    ..Default::default()
}, vec![]).unwrap();

stretch.compute_layout(node, Size::undefined());
`.trim();

const jsSnippet = `
import { Allocator, Node } from 'stretch-layout';

const allocator = new Allocator();
const node = new Node(allocator, {
    width: 100, 
    height: 100, 
});

const layout = node.computeLayout({
    width: undefined,
    height: undefined,
});
`.trim();

const kotlinSnippet = `
val node = Node(
    Style(
        size = Size(
            Dimension.Points(100f), 
            Dimension.Points(100f)
        )
    ), 
    listOf()
)

val layout = node.computeLayout(
    Size(null, null)
)
`.trim();

const swiftSnippet = `
let node = Node(
    style: Style(
        size: Size(
            width: .points(100.0), 
            height: .points(100.0)
        )
    ), 
    children: []
)
    
let layout = node.computeLayout(
    thatFits: Size(width: nil, height: nil)
)
`.trim();
  
const IndexPage = () => (
  <Layout>
    <SEO title="Stretch" keywords={[`rust`, `android`, `ios`, `web`, `flexbox`]} />

    <section className="hero-content">
        <div className="section-inner">
            <div className="title-letters">
                <img className="title-letter" src={S} />
                <img className="title-letter" src={T} />
                <img className="title-letter" src={R} />
                <img className="title-letter" src={E} />
                <img className="title-letter" src={T} />
                <img className="title-letter" src={C} />
                <img className="title-letter" src={H} />
            </div>

            <p>
                A high performance & cross-platform layout engine
            </p>

            <div className="badges">
                <a href="https://circleci.com/gh/vislyhq/stretch">
                    <img src="https://img.shields.io/circleci/project/github/vislyhq/stretch/master.svg?style=popout" />
                </a>

                <a href="https://crates.io/crates/stretch">
                    <img src="https://img.shields.io/crates/v/stretch.svg?style=popout" />
                </a>

                <a href="https://www.npmjs.com/package/stretch-layout">
                    <img src="https://img.shields.io/npm/v/stretch-layout.svg?style=popout" />
                </a>

                <a href="https://cocoapods.org/pods/StretchKit">
                    <img src="https://img.shields.io/cocoapods/v/StretchKit.svg?style=popout" />
                </a>

                <a href="https://bintray.com/visly/maven/stretch-kotlin-bindings">
                    <img src="https://api.bintray.com/packages/visly/maven/stretch-kotlin-bindings/images/download.svg?style=popout" />
                </a>
            </div>

            <div className="card-collection">
                <div className="card">
                    <h3>Written in Rust</h3>
                    <span>
                        We chose to write stretch in the{' '}
                        <a href="http://rust-lang.org">
                            Rust programming language
                        </a>{' '}
                        as it ensures memory safety, efficient
                        multi-threading, and has fantastic cross-platform
                        support.
                    </span>
                </div>

                <div className="card">
                    <h3>Optimised for mobile</h3>
                    <span>
                        While stretch can be used on any platform we chose
                        to optimize it for mobile. This means a small binary
                        size and minimal memory usage.
                    </span>
                </div>

                <div className="card">
                    <h3>Tested against Chrome</h3>
                    <span>
                        Stretch is tested against Chrome to ensure 100% web
                        compatibility. You can trust stretch to layout your
                        native apps exactly like your web apps.
                    </span>
                </div>
            </div>

            <div className="button-group">
                <Link to="/docs/rust" className="secondary">
                    Documentation
                </Link>
                <a href="https://github.com/vislyhq/stretch" className="primary">
                    View on GitHub
                </a>
            </div>
        </div>
    </section>

    <section className="cross-platform">
        <div className="section-inner">
            <h2>Cross platform</h2>
            <div className="code-blocks">
                <Code lang="rust" file="main.rs">{rustSnippet}</Code>
                <Code lang="javascript" file="index.js">{jsSnippet}</Code>
                <Code lang="swift" file="Main.swift">{swiftSnippet}</Code>
                <Code lang="kotlin" file="Main.kt">{kotlinSnippet}</Code>
            </div>
        </div>
    </section>

    <section className="featured-projects">
        <div className="section-inner">
            <h2>Featured projects</h2>
            <div className="card-collection">
                <div className="card">
                    <a href="https://visly.app">
                        <h3>
                            <img src={logo}/> Visly
                        </h3>
                    </a>
                    <span>
                        Visly is a design tool built from the ground up for
                        front-end engineers. Design components visually
                        which can later be imported directly into code.
                        Supports iOS, Android, and Web.{' '}
                        <a href="https://visly.app">
                            Learn more!
                        </a>
                    </span>
                </div>
                <div className="card">
                    <h3>Using Stretch?</h3>
                    <span>
                        If you're using stretch please{' '}
                        <a href="mailto:stretch@visly.app">
                            get in touch
                        </a>
                        ! We would love to learn more about the use cases
                        and challenges you are facing in your products. We
                        are happy to feature your product right here.
                    </span>
                </div>
            </div>
        </div>
    </section>

<section className="blog-posts">
    <div className="section-inner">
        <h2>Blog posts</h2>
        <div className="card-collection">
            <a href="https://medium.com/visly/stretch-a-flexbox-implementation-in-rust-60762b5a3331">
                <div className="card">
                    <h3>Introducing Stretch</h3>
                    <span>
                        Let’s talk about Layout. Layout is fundamental
                        to any UI application. The layout engine is what
                        takes a set of rules and figures out where to
                        place elements on the screen.
                    </span>
                </div>
            </a>
            <a href="https://medium.com/visly/rust-on-android-19f34a2fb43">
                <div className="card">
                    <h3>Rust on Android</h3>
                    <span>
                        You may have heard of rust, it’s a systems
                        programming language designed for memory safety
                        and speed. Built by Mozilla to power the next
                        generation high performance cross platform
                        software.
                    </span>
                </div>
            </a>
            <a href="https://medium.com/visly/rust-on-ios-39f799b3c1dd">
                <div className="card">
                    <h3>Rust on iOS</h3>
                    <span>
                        You may have heard of rust, it’s a systems
                        programming language designed for memory safety
                        and speed. Built by Mozilla to power the next
                        generation high performance cross platform
                        software.
                    </span>
                </div>
            </a>
        </div>
    </div>
</section>
  </Layout>
)

export default IndexPage
