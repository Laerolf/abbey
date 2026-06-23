# abbey-front

A Vue 3 web application for Abbey.

## Setup

### Project

```sh
pnpm install
```

#### Compile and Hot-Reload for Development

```sh
pnpm dev
```

#### Type-Check, Compile and Minify for Production

```sh
pnpm build
```

#### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
pnpm test:unit
```

#### Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```

### Browser

- Chromium-based browsers (Chrome, Edge, Brave, etc.):
  - [Vue.js devtools](https://chromewebstore.google.com/detail/vuejs-devtools/nhdogjmejiglipccpnnnanhbledajbpd)
  - [Turn on Custom Object Formatter in Chrome DevTools](http://bit.ly/object-formatters)
- Firefox:
  - [Vue.js devtools](https://addons.mozilla.org/en-US/firefox/addon/vue-js-devtools/)
  - [Turn on Custom Object Formatter in Firefox DevTools](https://fxdx.dev/firefox-devtools-custom-object-formatters/)

### Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) to make the TypeScript language service aware of `.vue` types.
