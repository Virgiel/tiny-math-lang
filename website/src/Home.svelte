<script>
  import { link } from 'svelte-routing';
  import LineExec from './LineExec.svelte';
  import Highlighter from './Highlighter.svelte';
</script>

<div class="header">
  <h1>Tiny Mathematical Language</h1>
  <a href="/editor" use:link>Try It</a>
</div>

<div class="column">
  <p class="pres">
    This project is a small experimentation with a mathematical language. The
    language implementation (lexer, parser, highlighter, interpreter) is written
    in rust, and this website and the code editor are written in Svelte.
  </p>
  <h2>Supported features</h2>
  <h3>Common operations</h3>
  <ul>
    <li>{'Addition      '} <LineExec code={'1+2'} /></li>
    <li>{'Subtraction   '} <LineExec code={'1-2'} /></li>
    <li>{'Multiplication'} <LineExec code={'1*2'} /></li>
    <li>{'Division      '} <LineExec code={'1/2'} /></li>
  </ul>
  <h3>Common functions</h3>
  <ul>
    <li>{'Cosinus '} <LineExec code={'cos(3)'} /></li>
    <li>{'Sinus   '} <LineExec code={'sin(1)'} /></li>
    <li>{'Log2    '} <LineExec code={'log2(100)'} /></li>
    <li>{'Log10   '} <LineExec code={'log2(100)'} /></li>
  </ul>
  <h3>Comments</h3>
  <Highlighter code={'# This is a comment'} />
</div>

<style>
  .header {
    background: #333333;
    padding: 16px 8px;
    text-align: center;
  }
  h1 {
    margin: 0;
    padding: 8px 0;
  }
  a {
    color: var(--blue);
  }
  a:active {
    color: var(--cyan);
  }
  li {
    white-space: pre;
    margin: 3px 0;
  }
  .column {
    width: 100%;
    max-width: 600px;
    padding: 8px;
    margin: auto;
  }
  p {
    text-align: justify;
    hyphens: auto;
  }
  @media (max-width: 600px) {
    .header {
      font-size: small;
    }
    p {
      font-size: 0.9em;
    }
  }
</style>
