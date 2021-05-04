<script>
  import { defaultCode } from './constant';
  import { heightsToGutterContent } from './gutter';
  import { saveSelection, restoreSelection } from './selection';
  import { debounce } from './utils';
  import { onMount } from 'svelte';

  export let wasm;

  let editor;
  let editorWrapper;
  let resultWrapper;
  let resultContent = '';
  let editorGutter = '';
  let resultGutter = '';

  function isCtrl(event) {
    return event.metaKey || event.ctrlKey;
  }

  function isUndo(event) {
    return isCtrl(event) && !event.shiftKey && event.key === 'z';
  }

  function isRedo(event) {
    return isCtrl(event) && event.shiftKey && event.key === 'z';
  }

  function isCopy(event) {
    return isCtrl(event) && event.key === 'c';
  }

  function isPaste(event) {
    return isCtrl(event) && event.key === 'v';
  }

  function isEdit(event) {
    return (
      !isUndo(event) &&
      !isRedo(event) &&
      !isPaste(event) &&
      !isCopy(event) &&
      event.key !== 'Meta' &&
      event.key !== 'Control' &&
      event.key !== 'Alt' &&
      !event.key.startsWith('Arrow')
    );
  }

  function onKeyDown(event) {
    // Prevent the creation of a div instead of a new line on enter
    if (event.key == 'Enter') {
      event.preventDefault();
      replaceSelection('\n');
    } else if (isEdit(event)) {
      refresh();
    }
  }

  function onPaste(e) {
    e.preventDefault();
    const text = e.clipboardData.getData('text/plain').replace(/\r/g, '');
    replaceSelection(text);
  }

  function replaceSelection(text) {
    const pos = saveSelection(editor);
    const content = editor.textContent;
    const code =
      content.substring(0, Math.min(pos.start, pos.end)) +
      text +
      content.substring(Math.max(pos.start, pos.end));
    syncEditorContent(code);

    const start = Math.min(pos.start, pos.end) + text.length;
    restoreSelection(editor, {
      start: start,
      end: start,
    });
  }

  function syncEditorContent(code) {
    const batchResult = wasm.execute_batch(code);
    resultGutter = heightsToGutterContent(batchResult.lines_height());
    resultContent = batchResult.content();
    const highlightResult = wasm.highlight_batch(code);
    editorGutter = heightsToGutterContent(highlightResult.lines_height());
    editor.innerHTML = highlightResult.content();
  }

  const refresh = debounce(() => {
    let pos = saveSelection(editor);
    syncEditorContent(editor.textContent);
    restoreSelection(editor, pos);
  }, 30);

  onMount(() => {
    refresh();
  });

  let isSyncing = false;
  function syncScroll(e) {
    if (isSyncing) {
      isSyncing = false;
      return;
    }
    const target = e.target;
    const topRate =
      target.scrollTop / (target.scrollHeight - target.clientHeight);
    const leftRate =
      target.scrollLeft / (target.scrollWidth - target.clientWidth);
    const other = target.classList.contains('bg')
      ? editorWrapper
      : resultWrapper;
    other.scrollTop = topRate * (other.scrollHeight - other.clientHeight);
    other.scrollLeft = leftRate * (other.scrollWidth - other.clientWidth);
    isSyncing = true;
  }
</script>

<div class="screen">
  <div class="wrapper" bind:this={editorWrapper} on:scroll={syncScroll}>
    <div class="gutter">{editorGutter}</div>
    <div
      class="editor"
      bind:this={editor}
      on:keydown={onKeyDown}
      on:paste={onPaste}
      contenteditable
      spellcheck="false"
    >
      {defaultCode}
    </div>
  </div>
  <div class="wrapper bg" bind:this={resultWrapper} on:scroll={syncScroll}>
    <div class="gutter bg">{resultGutter}</div>
    <div class="result">
      {@html resultContent}
    </div>
  </div>
</div>

<style>
  .screen {
    display: flex;
    height: 100vh;
    width: 100vw;
    flex-flow: row;
    line-height: 20px;
    tab-size: 4;
    white-space: pre;
  }

  @media (max-width: 800px) {
    .screen {
      flex-flow: column;
    }
  }
  .editor,
  .result {
    padding: 0 24px 100px 0;
    outline: none;
  }
  .gutter {
    height: 100%;
    position: sticky;
    left: 0;
    padding: 0px 8px;
    background: var(--background);
    color: var(--gutter);
  }
  .wrapper {
    padding: 8px 0;
    width: 100%;
    height: 100%;
    overflow: auto;
    position: relative;
    display: grid;
    grid-template-columns: auto 1fr;
  }
  .bg {
    background-color: #333333;
  }
  div::-webkit-scrollbar-track {
    background: none;
  }
  div::-webkit-scrollbar {
    width: 12px;
    height: 12px;
  }
  div::-webkit-scrollbar-corner {
    background-color: #92837490;
  }
  div::-webkit-scrollbar-thumb {
    background-color: #92837435;
  }
</style>
