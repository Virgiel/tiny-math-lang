<script>
  import { load } from './wasm';
  export let code = '';
</script>

<span class="code">
  {#await load()}
    {code} -> ?
  {:then wasm}
    {@html wasm.highlight(code)} -> {@html wasm.execute(code)}
  {/await}</span
>

<style>
  .code {
    background: #333333;
    border-radius: 4px;
    padding: 0 3px;
    width: fit-content;
    white-space: pre;
    width: 100%;
  }
</style>
