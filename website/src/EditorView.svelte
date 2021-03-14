<script>
  import { onMount, tick } from 'svelte';
  import { offsetInNodeAt, rectInNodeAt } from './offset';

  export let editor = null; // Editor logic
  export let sidePanel = false;

  const { lines, pos, onInput, onPos } = editor;

  let firstVisibleRow = 0;
  let lastVisibleRow = 0;
  let heights = []; // Line height mapping
  let gradualHeights = [];
  let editorWrapper;
  let contentWrapper;
  let focused = false;
  let editorHeight = 0;
  let editorWidth = 0;
  let averageHeight = 0;
  let sides = [];
  let contents = [];
  let pairs = [];
  let cursorPos = { x: 0, y: 0 };

  $: columnWidth = (editorWidth - gutterLen) / 2;
  $: scrollHeight = gradualHeights[gradualHeights.length - 1];
  $: items = $lines.map((line, index) => {
    let lineNbLen = Math.floor(Math.log10($lines.length)) + 1;
    let nb = '' + index;
    return {
      line: line,
      gutter: ' '.repeat(lineNbLen - nb.length) + nb,
    };
  });
  $: gutterLen = (Math.floor(Math.log10($lines.length)) + 1) * 9.6 + 16;
  $: maxLen = $lines.reduce(
    (max, line) => {
      max.content = Math.max(max.content, line.contentLen);
      max.side = Math.max(max.side, line.sideLen);
      return max;
    },
    { content: 0, side: 0 }
  );
  $: visibleRow = items
    .slice(firstVisibleRow, lastVisibleRow)
    .map((item, i) => {
      return { index: i + firstVisibleRow, data: item };
    });

  /** Ensure cursor visibility, adjusting scroll if necessary */
  function syncScroll() {
    const node = document.getElementsByClassName('cursor')[0];
    if (node == undefined || node == null) {
      return;
    }
    const cursor = node.getBoundingClientRect();
    const threeshold = 50;

    // Sync vertical scroll
    const editor = editorWrapper.getBoundingClientRect();
    let y = 0;
    const top = cursor.top - editor.top;
    if (top < threeshold) {
      y += top - threeshold;
    }
    const bottom = -cursor.bottom + editor.bottom;
    if (bottom < threeshold) {
      y -= bottom - threeshold;
    }
    if (y != 0) {
      editorWrapper.scrollBy(0, y);
    }

    // Sync horizontal scroll
    const scroller = contentWrapper.getBoundingClientRect();
    let x = 0;
    const left = cursor.left - scroller.left;
    if (left < threeshold) {
      x += left - threeshold;
    }
    const right = -cursor.right + scroller.right;
    if (right < threeshold) {
      x -= right - threeshold;
    }
    if (x != 0) {
      contentWrapper.scrollBy(x, 0);
    }
  }

  /** Find the text offset at pos in the nb index lines in editor */
  function findClickOffset(idx, x, y) {
    let row = contents[idx - firstVisibleRow];
    const nodeOffset = offsetInNodeAt(row, x, y);
    if (nodeOffset >= 0) {
      return nodeOffset;
    } else {
      return row.textContent.length;
    }
  }

  /** Find the line index pos */
  function lineIdxFromPos(y) {
    y += editorWrapper.scrollTop - editorWrapper.getBoundingClientRect().top;
    for (const [index, height] of heights.entries()) {
      if (y < height) {
        return index;
      } else {
        y -= height;
      }
    }
    return items.length;
  }

  function click(e) {
    const linePos = lineIdxFromPos(e.clientY);
    const charPos = findClickOffset(linePos, e.clientX, e.clientY);
    onPos(linePos, charPos);
  }

  $: {
    $pos, scrollAmount;
    syncCursor();
  }

  async function syncCursor() {
    await tick();
    const line = contents[$pos.linePos - firstVisibleRow];
    if (line) {
      const offset = Math.min(line.textContent.length, $pos.charPos);
      const rect = rectInNodeAt(line, offset);
      let x = 0;
      if (rect) {
        x = rect.x - gutterLen + 2;
      } else {
        x = line.getBoundingClientRect().left + 8 - gutterLen + 2;
      }
      cursorPos = {
        y: gradualHeights[$pos.linePos],
        x,
      };
      await tick();
      syncScroll();
    }
  }

  $: {
    $pos;
    syncScroll();
  }

  $: {
    lines, editorHeight;
    if (editorWrapper) {
      handleScroll();
    }
  }

  /** Keep visible row range in sync using the cached heights */
  async function handleScroll() {
    const { scrollTop } = editorWrapper;
    await tick();
    // Refresh heights
    for (let v = 0; v < visibleRow.length; v++) {
      heights[firstVisibleRow + v] = sidePanel
        ? Math.max(contents[v].offsetHeight, sides[v].offsetHeight)
        : pairs[v].offsetHeight;
    }
    let i = 0;
    let y = 0;
    // Search first visible row
    while (i < items.length && y + (heights[i] || averageHeight) < scrollTop) {
      y += heights[i] || averageHeight;
      i += 1;
    }
    firstVisibleRow = i;
    // Search last visible row
    while (i < items.length && y < scrollTop + editorHeight) {
      y += heights[i] || averageHeight;
      i += 1;
    }
    lastVisibleRow = i;

    // Try to guess heights
    averageHeight = y / lastVisibleRow;

    // Update gradual height
    let sum = 0;
    for (let i = 0; i < items.length; i++) {
      gradualHeights[i] = sum;
      sum += heights[i] || averageHeight;
    }
    gradualHeights[items.length] = sum;
  }

  let scrollAmount = 0;
  let scrollAmount2 = 0;
  function onVirtScroll(e) {
    scrollAmount = e.target.scrollLeft;
  }
  function onVirtScroll2(e) {
    scrollAmount2 = e.target.scrollLeft;
  }

  onMount(() => {
    contents = editorWrapper.getElementsByClassName('line content');
    sides = editorWrapper.getElementsByClassName('line side');
    pairs = editorWrapper.getElementsByClassName('pair');
    handleScroll();
    requestAnimationFrame(handleScroll);
  });
</script>

<div
  class="editor"
  bind:this={editorWrapper}
  on:keydown={onInput}
  on:click|preventDefault|stopPropagation={click}
  on:blur={() => (focused = false)}
  on:focus={() => (focused = true)}
  on:scroll={handleScroll}
  bind:offsetHeight={editorHeight}
  bind:offsetWidth={editorWidth}
  tabIndex="-1"
>
  <div class="gutter" style="width:{gutterLen}px; height:{scrollHeight}px;">
    {#each visibleRow as row (row.index)}
      <div
        class="gutter-item"
        style="height: {heights[row.index]}px;top: {gradualHeights[
          row.index
        ]}px;"
      >
        {row.data.gutter}
      </div>
    {/each}
  </div>
  <div class="lines" style="height:{scrollHeight}px; left:{gutterLen}px;">
    {#if sidePanel}
      {#each visibleRow as row (row.index)}
        <div
          class="pair line content"
          class:selected={row.index == $pos.linePos}
          style="top:{gradualHeights[row.index]}px;"
        >
          <span style="position:relative; left:{-scrollAmount + 8}px">
            {@html row.data.line.content}
          </span>
        </div>
        <div
          class="pair line side"
          style="top: {gradualHeights[row.index]}px; left:{columnWidth}px"
        >
          <span style="position:relative; left:{-scrollAmount2 + 8}px">
            {@html row.data.line.side}
          </span>
        </div>
      {/each}
    {:else}
      {#each visibleRow as row (row.index)}
        <div class="pair" style="top:{gradualHeights[row.index]}px;">
          <div class="line content" class:selected={row.index == $pos.linePos}>
            <span style="position:relative; left:{-scrollAmount + 8}px">
              {@html row.data.line.content}
            </span>
          </div>
          {#if row.data.line.side.length > 0}
            <div class="line side" style="left:{columnWidth}px">
              <span style="position:relative; left:{-scrollAmount + 8}px">
                {@html row.data.line.side}
              </span>
            </div>
          {/if}
        </div>
      {/each}
    {/if}
    {#if focused}
      <span
        class="cursor"
        style="positon: absolute; top:{cursorPos.y}px; left: {cursorPos.x}px;"
      />
    {/if}
  </div>
  {#if sidePanel}
    <div
      class="verticalScroller"
      bind:this={contentWrapper}
      style="width:{columnWidth}px; left:{gutterLen}px;"
      on:scroll={onVirtScroll}
    >
      <div class="widthWitness" style="width:{maxLen.content * 9.6 + 32}px;" />
    </div>
    <div
      class="verticalScroller"
      style="width:{columnWidth}px; left:{gutterLen + columnWidth}px;"
      on:scroll={onVirtScroll2}
    >
      <div class="widthWitness" style="width:{maxLen.side * 9.6 + 32}px;" />
    </div>
  {:else}
    <div
      class="verticalScroller"
      bind:this={contentWrapper}
      style="width:{columnWidth * 2}px; left:{gutterLen}px;"
      on:scroll={onVirtScroll}
    >
      <div class="widthWitness" style="width:{maxLen.content * 9.6 + 32}px;" />
    </div>
  {/if}
</div>

<style>
  .editor {
    display: grid;
    grid-template-columns: auto 1fr;
    line-height: 20px;
    overflow-y: overlay;
    overflow-x: hidden;
    outline: none;
    background-color: var(--background);
    position: relative;
    padding: 0;
  }
  .gutter {
    left: 0;
    top: 0;
    position: absolute;
    color: var(--grey);
    background: var(--background);
    text-align: right;
    white-space: pre;
    padding: 0 8px;
    z-index: 1;
  }
  .selected {
    background: var(--highlight);
  }
  .gutter-item {
    position: absolute;
  }
  .lines {
    position: absolute;
    top: 0;
    padding-left: 8px;
    overflow: hidden;
    width: 100%;
  }
  .line.side {
    background-color: #333333;
  }
  .pair {
    position: absolute;
    width: 100%;
  }
  .line {
    color: var(--foreground);
    width: 100%;
    line-height: 20px;
    min-height: 20px;
    white-space: pre;
    overflow: hidden;
  }
  .verticalScroller {
    position: sticky;
    top: 0;
    overflow-x: auto;
    z-index: 666;
    height: 100%;
  }
  .widthWitness {
    height: 40px;
  }
  .cursor {
    position: absolute;
    width: 2px;
    margin-left: -2px;
    height: 20px;
    background: var(--cyan);
    animation: blink 0.7s ease-out infinite;
    animation-direction: alternate;
    animation-delay: 1s;
  }
  @keyframes blink {
    0% {
      opacity: 1;
    }
    100% {
      opacity: 1;
    }
  }
  div::-webkit-scrollbar-track {
    background: none;
  }
  div::-webkit-scrollbar {
    width: 12px;
  }
  div::-webkit-scrollbar-thumb {
    background-color: #92837435;
  }
</style>
