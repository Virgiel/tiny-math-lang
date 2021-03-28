<script>
  import { Router, Route } from 'svelte-routing';
  import Home from './Home.svelte';
  import Loading from './Loading.svelte';

  export let url = '';
</script>

<Router {url}>
  <Route path="editor" component={Loading} />
  <Route path="/" component={Home} />
</Router>
