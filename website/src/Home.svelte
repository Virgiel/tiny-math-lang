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
  <h2>Syntax</h2>
  <h3>Expressions</h3>
  <p><LineExec code={'1+2+3+4+5+6+7+8+9'} /></p>
  <h3>Functions</h3>
  <p><LineExec code={'sqrt(49)'} /></p>
  <h3>Variables</h3>
  <p><LineExec code={'square49 = sqrt(49)'} /></p>
  <h3>Comments</h3>
  <Highlighter code={'# This is a useless comment'} />
  <h3>String</h3>
  <p><LineExec code={'"Hello world"'} /></p>
  <p><LineExec code={'"The answer is " 42'} /></p>
  <h2>Supported features</h2>
  <h3>Common operations</h3>
  <ul>
    <li>{'Addition      '} <LineExec code={'1 + 2'} /></li>
    <li>{'Subtraction   '} <LineExec code={'1 - 2'} /></li>
    <li>{'Multiplication'} <LineExec code={'1 * 2'} /></li>
    <li>{'Division      '} <LineExec code={'1 / 2'} /></li>
    <li>{'Modulo        '} <LineExec code={'100 % 7'} /></li>
  </ul>
  <h3>Common constants</h3>
  <ul>
    <li>{"Archimedes' constant (π) "} <LineExec code={'PI'} /></li>
    <li>{"Euler's number (ℇ)       "} <LineExec code={'E'} /></li>
  </ul>
  <h3>Common functions</h3>
  <ul>
    <li>{'Floor   '} <LineExec code={'floor(10/3)'} /></li>
    <li>{'Ceil    '} <LineExec code={'ceil(10/3)'} /></li>
    <li>{'Round   '} <LineExec code={'round(10/3)'} /></li>
  </ul>
  <ul>
    <li>{'Trunc   '} <LineExec code={'trunc(13/2)'} /></li>
    <li>{'Fract   '} <LineExec code={'fract(13/2)'} /></li>
  </ul>
  <ul>
    <li>{'Sqrt    '} <LineExec code={'sqrt(49)'} /></li>
    <li>{'Exp     '} <LineExec code={'exp(0)'} /></li>
    <li>{'Ln      '} <LineExec code={'ln(1)'} /></li>
  </ul>
  <ul>
    <li>{'Log2    '} <LineExec code={'log2(2048)'} /></li>
    <li>{'Log10   '} <LineExec code={'log10(100)'} /></li>
  </ul>
  <ul>
    <li>{'Cosinus '} <LineExec code={'cos(0)'} /></li>
    <li>{'Sinus   '} <LineExec code={'sin(0)'} /></li>
    <li>{'Tangente'} <LineExec code={'tan(0)'} /></li>
    <li>{'ACosinus'} <LineExec code={'acos(cos(0))'} /></li>
    <li>{'ASinus  '} <LineExec code={'asin(sin(0))'} /></li>
    <li>{'ATan    '} <LineExec code={'atan(tan(0))'} /></li>
  </ul>
  <h2>TO DO</h2>
  <ul>
    <li>Defined functions?</li>
    <li>Formatter?</li>
  </ul>
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
  p {
    margin: 3px 0;
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
