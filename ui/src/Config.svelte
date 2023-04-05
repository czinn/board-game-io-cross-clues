<script>
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();

export let config;
export let readonly;

let rows = config.size.row;
let cols = config.size.col;
let word_lists = config.word_lists;
let custom_words = config.custom_words.join(', ');

function config_changed(config) {
  rows = config.size.row;
  cols = config.size.col;
  word_lists = config.word_lists;
  custom_words = config.custom_words.join(', ');
}

$: config_changed(config);

function update_config() {
  dispatch('config_change', {
    size: {row: rows, col: cols},
    word_lists,
    custom_words: custom_words.split(/\s*,\s*/),
  });
}
</script>

<div class="content">
  {#if config}
    {#if !readonly}
      Rows: <input type="number" bind:value={rows} min=3 max=10 on:change={update_config}/><br>
      Columns: <input type="number" bind:value={cols} min=3 max=10 on:change={update_config}/>
    {/if}
    <p>{rows} rows by {cols} columns</p>
    <h3>Word lists</h3>
    <ul>
      {#each word_lists as word_list}
        <li>{word_list[0]} <input type=checkbox disabled={readonly} bind:checked={word_list[1]} on:change={update_config}/></li>
      {/each}
    </ul>
    <textarea disabled={readonly} bind:value={custom_words} on:change={update_config}/>
  {/if}
</div>

<style>
.content {
  padding: 0.5em 1em;
}
</style>
