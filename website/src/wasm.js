import wasm from '../../wasm/Cargo.toml';

let loaded;
const loader = wasm().then(it => {
  loaded = it;
  return it;
});

async function load() {
  return await loader;
}

function crate() {
  return loaded;
}

export { load, crate };
