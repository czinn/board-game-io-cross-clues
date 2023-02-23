<script>
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();

export let config;
export let readonly;

let rows = config.size.row;
let cols = config.size.col;

function config_changed(config) {
  rows = config.size.row;
  cols = config.size.col;
}

$: config_changed(config);

function update_config() {
  dispatch('config_change', {'size': {'row': rows, 'col': cols}});
}
</script>

<div class="content">
  {#if config}
    {#if !readonly}
      Rows: <input type="number" bind:value={rows} min=3 max=10 on:change={() => update_config()}/><br>
      Columns: <input type="number" bind:value={cols} min=3 max=10 on:change={() => update_config()}/>
    {/if}
    <p>{rows} rows by {cols} columns</p>
  {/if}
</div>

<style>
.content {
  padding: 0.5em 1em;
}
</style>
