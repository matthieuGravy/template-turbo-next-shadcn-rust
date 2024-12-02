# Turborepo Tailwind CSS starter

This is an unofficial starter Turborepo with additional features.

## Using this example

Run the following command:

```sh
git clone git@github.com:matthieuGravy/template-turbo-next-shadcn-rust.git
cd template-turbo-next-shadcn-rust
git remote remove origin
```

```sh
# Install dependencies
npm install
# or
pnpm install
# or
yarn install
```

```sh
# start all apps
turbo run dev
```

Note: You'll need to manually add the following features after installation:

- [shadcn/ui](https://ui.shadcn.com/) components
- Rust integration

## What's inside?

This Turborepo includes the following packages/apps:

### Apps and Packages

- `docs`: a [Next.js](https://nextjs.org/) app with [Tailwind CSS](https://tailwindcss.com/) and shadcn/ui
- `web`: another [Next.js](https://nextjs.org/) app with [Tailwind CSS](https://tailwindcss.com/) and shadcn/ui
- `ui`: a React component library with [Tailwind CSS](https://tailwindcss.com/) and shadcn/ui shared by both applications
- `@repo/eslint-config`: `eslint` configurations (includes `eslint-config-next` and `eslint-config-prettier`)
- `@repo/typescript-config`: `tsconfig.json`s used throughout the monorepo

Each package/app is 100% [TypeScript](https://www.typescriptlang.org/).

### Building packages/ui

This example is set up to produce compiled styles for `ui` components into the `dist` directory. The component `.tsx` files are consumed by the Next.js apps directly using `transpilePackages` in `next.config.js`. This was chosen for several reasons:

- Make sharing one `tailwind.config.js` to apps and packages as easy as possible.
- Make package compilation simple by only depending on the Next.js Compiler and `tailwindcss`.
- Ensure Tailwind classes do not overwrite each other. The `ui` package uses a `ui-` prefix for it's classes.
- Maintain clear package export boundaries.

Another option is to consume `packages/ui` directly from source without building. If using this option, you will need to update the `tailwind.config.js` in your apps to be aware of your package locations, so it can find all usages of the `tailwindcss` class names for CSS compilation.

For example, in [tailwind.config.js](packages/tailwind-config/tailwind.config.js):

```js
  content: [
    // app content
    `src/**/*.{js,ts,jsx,tsx}`,
    // include packages if not transpiling
    "../../packages/ui/*.{js,ts,jsx,tsx}",
  ],
```

If you choose this strategy, you can remove the `tailwindcss` and `autoprefixer` dependencies from the `ui` package.

### Utilities

This Turborepo has some additional tools already setup for you:

- [Tailwind CSS](https://tailwindcss.com/) for styles
- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

### Additional Features

This template includes several enhancements to the original Turborepo starter:

- [shadcn/ui](https://ui.shadcn.com/) for beautiful, accessible components
- Rust integration for enhanced performance

## Support this project

If you find this project useful, please consider giving it a star on GitHub. It helps others discover the project and shows your support!

[![GitHub Stars](https://img.shields.io/github/stars/matthieuGravy/template-turbo-next-shadcn-rust.svg?style=social&label=Star)](https://github.com/matthieuGravy/template-turbo-next-shadcn-rust)

Feel free to share it with your friends and colleagues!
