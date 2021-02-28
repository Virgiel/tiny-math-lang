<script>
  import EditorView from './EditorView.svelte';
  import { newMultilineEditor } from './multineEditor';

  import { load } from './wasm';
  async function editor() {
    await load();
    return newMultilineEditor();
  }
</script>

<div class="screen">
  {#await editor()}
    <p>Loading ...</p>
  {:then editor}
    <EditorView {editor} />
  {/await}
</div>

<style>
  .screen {
    display: grid;
    height: 100vh;
    width: 100vw;
  }
  p {
    margin: auto;
    font-size: 1.5em;
    font-weight: bold;
    text-align: center;
  }
</style>
