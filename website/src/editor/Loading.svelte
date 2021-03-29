<script>
  import Editor from './Editor.svelte';
  import { load } from '../wasm';
</script>

{#await load()}
  <p>Loading...</p>
{:then wasm}
  <Editor {wasm} />
{/await}

<style>
  p {
    width: 100%;
    text-align: center;
  }
</style>
