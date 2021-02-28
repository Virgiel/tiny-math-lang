<script>
  import { Router, Route } from 'svelte-routing';
  import Home from './Home.svelte';
  import Editor from './Editor.svelte';

  export let url = '';
</script>

<Router {url}>
  <Route path="editor" component={Editor} />
  <Route path="/" component={Home} />
</Router>
