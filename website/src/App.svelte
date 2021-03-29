<script>
  import { Router, Route } from 'svelte-routing';
  import Home from './home/Home.svelte';
  import Loading from './editor/Loading.svelte';

  export let url = '';
</script>

<Router {url}>
  <Route path="editor" component={Loading} />
  <Route path="/" component={Home} />
</Router>
