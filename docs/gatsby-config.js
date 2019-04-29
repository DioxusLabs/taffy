module.exports = {
  pathPrefix: "/stretch",
  siteMetadata: {
    title: `Stretch`,
    description: `A high performance & cross-platform layout engine`,
    author: `@vislyhq`,
  },
  plugins: [
    `gatsby-plugin-react-helmet`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `images`,
        path: `${__dirname}/src/images`,
      },
    },
    `gatsby-transformer-sharp`,
    `gatsby-plugin-sharp`,
    `gatsby-mdx`,
  ],
}
