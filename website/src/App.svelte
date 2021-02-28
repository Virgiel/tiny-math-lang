<script>
  import EditorView from './EditorView.svelte';
  import { newMultilineEditor } from './multineEditor';

  const multiline = newMultilineEditor();
</script>

<div class="screen">
  <EditorView editor={multiline} />
</div>

<style>
  .screen {
    display: grid;
    height: 100vh;
    width: 100vw;
  }
</style>
