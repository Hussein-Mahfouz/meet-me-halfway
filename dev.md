# Developer guide

To run this, you'll need `node` and [wasm-pack](https://github.com/rustwasm/wasm-pack) installed. Clone the repo, `cd web`, then:

- `npm i` to install node dependencies -- you only need to do this once, or when `package.json` changes
- `npm run wasm` to build the Rust `backend/` code. You need to do this at least once, and anytime that code changes -- it doesn't automatically get rebuilt.
- `npm run dev` to run the web app locally, after the two steps above are done. Changes to the Svelte `web/` code will auto-reload and your browser will auto-refresh, but sometimes this gets a bit stuck and you have to hard-refresh in your browser.
- `npm run fmt` to auto-format the code
- `npm run check` to check TypeScript warnings and errors
