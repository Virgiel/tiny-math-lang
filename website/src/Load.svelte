<script>
  import App from './App.svelte';

  import { load } from './wasm';
</script>

{#await load()}
  <div>
    <p>Loading ...</p>
  </div>
{:then}
  <App />
{/await}

<style>
  div {
    height: 100vh;
    width: 100vw;
    display: flex;
  }
  p {
    margin: auto;
    font-size: 1.5em;
    font-weight: bold;
    text-align: center;
  }
</style>
