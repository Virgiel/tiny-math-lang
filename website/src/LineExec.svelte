<script>
  import { load } from './wasm';
  export let code = '';
  async function cmp() {
    const wasm = await load();
    return {
      code: wasm.highlight(code),
      result: wasm.execute(code),
    };
  }
</script>

<span class="code">
  {#await cmp()}
    {code}
  {:then cmp}
    {@html cmp.code}{#if cmp.result.length > 0}
      {' ->'} {@html cmp.result}
    {/if}
  {/await}</span
>

<style>
  .code {
    background: #333333;
    border-radius: 4px;
    padding: 0 4px;
    width: fit-content;
    white-space: pre;
    width: 100%;
  }
</style>
