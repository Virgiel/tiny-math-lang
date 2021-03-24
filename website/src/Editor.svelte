<script>
  import { onMount } from 'svelte';
  import { CodeJar } from 'codejar';
  import { load } from './wasm';
  import LineExec from './LineExec.svelte';

  let editor;
  let result;
  let resultContent = 'Loading...';

  const defaultCode = `
# Welcome into the Tiny Mathematic Language online editor

# You can perform simple operations
1+1
1+2+3+4+5+6+7+8+9
3*3
(3/8) * (32+4) + 34
2+2
8/(2*(2+2))

# Use constants
PI
cos(PI)
cos(PI/2)
sin(23*PI)

# Use mathematical function
cos(5)
sin(4/8)
log2(1000)
exp(ln(7))
10 / 3
trunc(10/3)
fract(10/3)

# And print text

"J'aime le chocolat"
"Full: "1/3"   Truncated: "trunc(1/3)"    Decimal: "fract(1/3)

# And now somethings more interesting
a = 12
b = 32
"a = "a" & b = "b
"hypotenuse = sqrt(a*a+b*b)"
"hypotenuse = "sqrt(a*a+b*b)

`;

  let editorGutter = '';

  onMount(() => {
    load().then(wasm => {
      let isInit = false;
      const jar = CodeJar(editor, editor => {
        if (!isInit) {
          editor.style.whiteSpace = 'pre';
          editor.style.resize = 'none';
          editor.style.overflow = 'visible';
        }
        let gutter = '';
        editor.innerHTML = wasm
          .highlight_batch(editor.textContent)
          .map((line, idx) => {
            gutter += `${idx}\n`;
            return line;
          })
          .join('\n');
        editorGutter = gutter;
      });
      const onUpdate = code => {
        resultContent = wasm.execute_batch(code).join('\n');
      };
      jar.onUpdate(onUpdate);
      onUpdate(defaultCode);
    });
  });

  let isSyncing = false;
  function syncScroll(e) {
    if (isSyncing) {
      isSyncing = false;
      return;
    }
    const target = e.target;
    const scrollRate = target.scrollTop / target.scrollHeight;
    const other = target.classList.contains('editor') ? result : editor;
    other.scrollTop = scrollRate * other.scrollHeight;
    isSyncing = true;
  }
</script>

<div class="screen">
  <div class="wrapper">
    <div class="gutter">{editorGutter}</div>
    <div class="editor" bind:this={editor} on:scroll={syncScroll} resize="none">
      {defaultCode}
    </div>
  </div>
  <div class="wrapper bg">
    <div class="gutter bg">{editorGutter}</div>
    <div class="result" bind:this={result} on:scroll={syncScroll}>
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
  .gutter {
    height: 100%;
    position: sticky;
    left: 0;
    padding: 0 8px;
    background: var(--background);
    color: var(--grey);
  }
  .wrapper {
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
