/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation
 The sidebars can be generated from the filesystem, or explicitly defined here.
 Create as many sidebars as you want.
 */

module.exports = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  // conceptsSidebar: [{type: 'autogenerated', dirName: '.'}],

  // But you can create a sidebar manually
  sidebar: [
    {
      type: "category",
      label: "Getting Started",
      link: { type: "doc", id: "getting-started/introduction" },
      items: [
        "getting-started/build-a-sample-app",
        "getting-started/examples",
        "getting-started/editor-setup",
      ],
    },
    {
      type: "category",
      label: "Concepts",
      link: {
        type: "generated-index",
        title: "Yew concepts",
        description: "Learn about the important Yew concepts!",
      },
      items: [
        {
          type: "category",
          label: "Components",
          link: { type: "doc", id: "concepts/components/introduction" },
          items: [
            "concepts/components/lifecycle",
            "concepts/components/scope",
            "concepts/components/callbacks",
            "concepts/components/properties",
            "concepts/components/children",
            "concepts/components/refs",
          ],
        },
        {
          type: "category",
          label: "HTML",
          link: { type: "doc", id: "concepts/html/introduction" },
          items: [
            "concepts/html/components",
            "concepts/html/elements",
            "concepts/html/events",
            "concepts/html/classes",
            "concepts/html/fragments",
            "concepts/html/lists",
            "concepts/html/literals-and-expressions",
            "concepts/html/conditional-rendering",
          ],
        },
        {
          type: "category",
          label: "Function Components",
          items: [
            "concepts/function-components/introduction",
            "concepts/function-components/attribute",
            "concepts/function-components/pre-defined-hooks",
            "concepts/function-components/custom-hooks",
          ],
        },
        {
          type: "category",
          label: "wasm-bindgen",
          link: {
            type: "generated-index",
            title: "wasm-bindgen",
            description: "Learn about wasm-bindgen",
            slug: "/concepts/wasm-bindgen",
          },
          items: [
            "concepts/wasm-bindgen/introduction",
            "concepts/wasm-bindgen/web-sys",
          ],
        },
        "concepts/agents",
        "concepts/contexts",
        "concepts/router",
        "concepts/suspense",
      ],
    },
    {
      type: "category",
      label: "Advanced topics",
      link: {
        type: "generated-index",
        title: "Advanced topics",
        description:
          "Learn about the advanced topics and inner workings of Yew!",
      },
      items: [
        "advanced-topics/how-it-works",
        "advanced-topics/optimizations",
        "advanced-topics/portals",
        "advanced-topics/server-side-rendering",
      ],
    },
    {
      type: "category",
      label: "More",
      link: {
        type: "generated-index",
        title: "Miscellaneous",
      },
      items: [
        "more/debugging",
        "more/external-libs",
        "more/css",
        "more/testing",
        "more/roadmap",
      ],
    },
    {
      type: "category",
      label: "Migration guides",
      items: [
        {
          type: "category",
          label: "yew",
          items: ["migration-guides/yew/from-0_18_0-to-0_19_0"],
        },
        {
          type: "category",
          label: "yew-agent",
          items: ["migration-guides/yew-agent/from-0_0_0-to-0_1_0"],
        },
        {
          type: "category",
          label: "yew-router",
          items: ["migration-guides/yew-router/from-0_15_0-to-0_16_0"],
        },
      ],
    },
    "tutorial",
  ],
};
