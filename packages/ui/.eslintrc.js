module.exports = {
  extends: ["@repo/eslint-config/react.js"],
  settings: {
    "import/resolver": {
      typescript: {
        project: "./tsconfig.json"
      },
      node: {
        paths: ["src"],
        extensions: [".js", ".jsx", ".ts", ".tsx"]
      }
    }
  },
  rules: {
    "import/no-extraneous-dependencies": "off"  // Désactivé

  }
};