import wasm from '../../wasm/Cargo.toml';

let loaded;

async function load() {
  loaded = await wasm();
  return loaded;
}

function crate() {
  return loaded;
}

export { load, crate };
