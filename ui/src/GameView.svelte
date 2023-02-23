<script>
import { createEventDispatcher } from 'svelte';
import { user, view, player_id } from './stores.js';

const dispatch = createEventDispatcher();
</script>

<div class="content">
  <h1>Hello</h1>
  { JSON.stringify($view) }
</div>

<style>
.content {
  padding: 0.5em 1em;
}
</style>
