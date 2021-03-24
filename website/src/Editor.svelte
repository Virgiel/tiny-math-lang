<script>
  import { onMount } from 'svelte';
  import { CodeJar } from 'codejar';
  import { load } from './wasm';

  let editor;
  let result = 'Loading...';

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
  let lineHeight = [];

  onMount(() => {
    load().then(wasm => {
      let isInit = false;
      const jar = CodeJar(editor, editor => {
        if (!isInit) {
          editor.style.whiteSpace = 'pre';
          editor.style.resize = 'none';
        }
        editor.innerHTML = wasm
          .highlight_batch(editor.textContent)
          .map((line, idx) => {
            let height = lineHeight[idx] | 1;
            if (height > 1) {
              return (
                line +
                `<span class="spacer" style="line-height:${
                  height * 20
                }px;">\n</span>`
              );
            } else {
              return line + '\n';
            }
          })
          .join('');
      });
      const onUpdate = code => {
        result = wasm
          .execute_batch(code)
          .map((line, idx) => {
            lineHeight[idx] = line.split('\n').length;
            return line;
          })
          .join('\n');
        console.log(lineHeight);
      };
      jar.onUpdate(onUpdate);
      onUpdate(defaultCode);
    });
  });
</script>

<div class="screen">
  <div class="editor" bind:this={editor} resize="none">
    {defaultCode}
  </div>
  <div class="result">
    {@html result}
  </div>
</div>

<style>
  .screen {
    display: flex;
    height: 100vh;
    width: 100vw;
    flex-flow: row;
  }
  .screen * {
    height: 100vh;
    width: 50vw;
  }

  @media (max-width: 800px) {
    .screen {
      flex-flow: column;
    }
    .screen * {
      height: 50vh;
      width: 100vw;
    }
  }
  .screen * {
    overflow: auto;
    line-height: 20px;
    padding: 10px;
    tab-size: 4;
    white-space: pre;
  }
  .result {
    background-color: #333333;
  }
  div::-webkit-scrollbar-track {
    background: none;
  }
  div::-webkit-scrollbar {
    width: 12px;
  }
  div::-webkit-scrollbar-corner {
    background-color: #92837490;
  }
  div::-webkit-scrollbar-thumb {
    background-color: #92837435;
  }
</style>
