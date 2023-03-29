<script>
import { createEventDispatcher } from 'svelte';
import { user, view, player_id } from './stores.js';
import { row_labels, col_labels } from './game_stores.js';
import PlayerName from './components/PlayerName.svelte';

const dispatch = createEventDispatcher();

let clue_to_give = "";

function clue_is_valid(clue_to_give) {
  return clue_to_give.length > 0 && !clue_to_give.includes(" ");
}

function give_clue() {
  if (clue_is_valid(clue_to_give)) {
    dispatch('do_action', {'type': 'give_clue', 'clue': clue_to_give });
    clue_to_give = "";
  }
}

function tap_tile(row, col) {
  dispatch('do_action', {'type': 'tap_tile', 'tile': {'row': row, 'col': col}});
}

function col_letter(col) {
  return String.fromCharCode('A'.charCodeAt(0) + col);
}

function tile_coords(tile) {
  return col_letter(tile.col) + (tile.row + 1);
}

function find_clue(row, col) {
  for (let i = 0; i < $view.bad_clues.length; i++) {
    if ($view.bad_clues[i].tile.row === row && $view.bad_clues[i].tile.col === col) {
      return $view.bad_clues[i];
    }
  }
  return "";
}

function toggle_vote(row, col) {
  const current_vote = $view.votes[row][col].includes($player_id);
  dispatch('do_action', {'type': 'set_vote', 'tile': {'row': row, 'col': col}, 'vote': !current_vote});
}
</script>

<div class="content">
  <h1>Cross Clues</h1>
  {#if $view.players_with_tiles.length === 0}
    <h2>Score: { $view.size.row * $view.size.col - $view.bad_clues.length }</h2>
  {/if}
  {#if $view.player_tile !== null}
    <p>Your tile is <strong>{ tile_coords($view.player_tile) }</strong> ({$col_labels[$view.player_tile.col]}, {$row_labels[$view.player_tile.row]})</p>
  {/if}
  {#if $view.player_tile !== null && ($view.current_clue === null || $view.current_clue.player !== $player_id)}
    <form on:submit|preventDefault={() => give_clue()}>
      <input bind:value={clue_to_give}/>
      <button type='submit' disabled={!clue_is_valid(clue_to_give) || $view.current_clue !== null}>
        Give Clue
      </button>
    </form>
  {/if}
  {#if $view.current_clue !== null}
    <p class='clue'><PlayerName player_id={$view.current_clue.player}/> gave the clue <strong>{$view.current_clue.clue}</strong></p>
  {/if}
  <table class='cards'>
    <tr>
      <td></td>
      {#each $col_labels as col_label, col}
        <td class='col-label'>{ col_letter(col) }<br/>{ col_label }</td>
      {/each}
    </tr>
    {#each $row_labels as row_label, row}
      <tr>
        <td class='row-label'>{row + 1}<br/>{ row_label }</td>
        {#each $col_labels as col_label, col}
          <td class='card' on:click={() => toggle_vote(row, col)}>
            {#if $view.good_clues[row][col] !== null}
              <p class='good-clue'>{ $view.good_clues[row][col].clue }</p>
            {:else}
              {#each $view.votes[row][col] as voter, i}
                {#if i > 0}, {/if}<PlayerName player_id={voter}/>
              {/each}
              {#if $view.current_clue !== null && $view.current_clue.player !== $player_id}
                <button on:click={() => tap_tile(row, col)}>👆</button>
              {/if}
              {#if $view.players_with_tiles.length === 0}
                <p class='bad-clue'>{ find_clue(row, col).clue }</p>
              {/if}
            {/if}
          </td>
        {/each}
      </tr>
    {/each}
  </table>
  {#if $view.players_with_tiles.length !== $view.players.length && $view.players_with_tiles.length > 0}
    <hr>
    <h2>Remaining players with clues to give</h2>
    {#each $view.players_with_tiles as player, i}
      {#if i > 0}, {/if}<PlayerName player_id={player}/>
    {/each}
  {/if}
  {#if $view.bad_clues.length > 0}
    <hr>
    <h2>Incorrect guesses</h2>
    <table class='bad-guesses'>
      <tr>
        <th>Player</th>
        <th>Clue</th>
        <th>Guesser</th>
        <th>Guess</th>
        {#if $view.bad_clues[0].tile !== null}
          <th>Actual</th>
        {/if}
      </tr>
      {#each $view.bad_clues as bad_clue}
        <tr>
          <td><PlayerName player_id={bad_clue.player}/></td>
          <td>{ bad_clue.clue }</td>
          <td><PlayerName player_id={bad_clue.guesser}/></td>
          <td>{ tile_coords(bad_clue.guess) }</td>
          {#if bad_clue.tile !== null}
            <td>{ tile_coords(bad_clue.tile) }</td>
          {/if}
        </tr>
      {/each}
    </table>
  {/if}
</div>

<style>
.content {
  padding: 0.5em 1em;
}

table.bad-guesses {
  margin: 0 auto;
}

.bad-guesses th {
  border-bottom: 1px solid white;
}

.bad-guesses td {
  padding: 0 0.5em;
}

table.cards {
  margin: 0 auto;
}

td.col-label {
  padding: 0.5em;
  font-size: 1.5em;
}

td.row-label {
  padding: 0.5em;
  font-size: 1.5em;
  text-align: right;
}

td.card {
  width: 8em;
  height: 8em;
  max-width: 8em;
  max-height: 8em;
  border: 1px solid white;
  word-wrap: break-word;
  position: relative;
  user-select: none;
}

td.card button {
  position: absolute;
  bottom: -0.25em;
  right: -0.25em;
  padding: 0.25em;
}

p.clue {
  font-size: 1.5em;
}

span.number {
  margin-right: 1em;
  text-align: left;
}

.good-clue {
  color: #00FF00;
  font-size: 1.5em;
}

.bad-clue {
  color: #FF0000;
  font-size: 1.5em;
}
</style>
