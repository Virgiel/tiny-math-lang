<script>
  import { onMount } from 'svelte';
  import { CodeJar } from 'codejar';
  import { load } from './wasm';
  import { defaultCode } from './constant';
  import { linesToGutterContent } from './gutter';

  let editor;
  let editorWrapper;
  let resultWrapper;
  let resultContent = 'Loading...';
  let editorGutter = '';
  let resultGutter = '';

  onMount(() => {
    editorGutter = linesToGutterContent(defaultCode.split('\n'));
    load().then(wasm => {
      const jar = CodeJar(editor, editor => {
        const lines = wasm.highlight_batch(editor.textContent);
        editorGutter = linesToGutterContent(lines);
        editor.innerHTML = lines.join('\n') + '\n';
      });
      const onUpdate = code => {
        const lines = wasm.execute_batch(code);
        resultGutter = linesToGutterContent(lines);
        resultContent = lines.join('\n');
      };
      jar.onUpdate(onUpdate);
      onUpdate(defaultCode);
      editor.style.whiteSpace = 'pre';
      editor.style.resize = 'none';
      editor.style.overflow = 'visible';
    });
  });

  let isSyncing = false;
  function syncScroll(e) {
    if (isSyncing) {
      isSyncing = false;
      return;
    }
    const target = e.target;
    const topRate = target.scrollTop / target.scrollHeight;
    const leftRate = target.scrollLeft / target.scrollWidth;
    const other = target.classList.contains('bg')
      ? editorWrapper
      : resultWrapper;
    other.scrollTop = topRate * other.scrollHeight;
    other.scrollLeft = leftRate * other.scrollWidth;
    isSyncing = true;
  }
</script>

<div class="screen">
  <div class="wrapper" bind:this={editorWrapper} on:scroll={syncScroll}>
    <div class="gutter">{editorGutter}</div>
    <div class="editor" bind:this={editor}>
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
