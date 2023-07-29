<script>
import BoardGameClient from './BoardGameClient.svelte'
import {
  user_map,
  player_map,
  view,
  config,
  user,
  user_id,
  users,
  room_id,
} from './stores.ts';

export let game_name;

let client;
let connecting;
let reconnect_tokens = [];

let new_username;
let go_to_room_id;

function config_handler(event) {
  client.handle_config_update(event.detail);
}

function do_action(event) {
  client.do_action(event.detail);
}

let admin_panel = false;
let reassigning = null;

function kick_user(user_id) {
  client.kick_user(user_id);
}

function reassign_player(user_id) {
  reassigning = user_id;
}

function reassign_to(user_id) {
  client.reassign_player(reassigning, user_id);
  reassigning = null;
}

function reset_to_lobby() {
  client.reset_to_lobby();
}

function get_addr() {
  if (import.meta.env.DEV) {
    return 'ws://localhost:9002';
  } else {
    const loc = window.location;
    let new_uri;
    if (loc.protocol === 'https:') {
        new_uri = 'wss:';
    } else {
        new_uri = 'ws:';
    }
    new_uri += '//' + loc.host + '/ws';
    return new_uri;
  }
}

</script>

<main>
  <BoardGameClient
    bind:this={client}
    bind:connecting={connecting}
    bind:reconnect_tokens={reconnect_tokens}
    addr={get_addr()} />
  {#if $view === null}
    <h1>{game_name}</h1>
  {/if}
  {#if connecting}
    <p>Connecting...</p>
  {:else}
    {#if $user_id === null}
      <form on:submit|preventDefault={() => client.join_room(new_username)}>
        <input placeholder="Username" bind:value={new_username} autocomplete="username">
        <button type="submit">
          {#if $room_id === null}
            Create Room
          {:else}
            Join Room
          {/if}
        </button>
      </form>
      {#if $room_id === null}
        <form on:submit|preventDefault={() => client.go_to_room(go_to_room_id)}>
          <input placeholder="Room code" bind:value={go_to_room_id}>
          <button type="submit">Join Room</button>
        </form>
      {/if}
      {#each reconnect_tokens as token}
        <button on:click={() => client.rejoin_room(token)}>Rejoin as {token.username}</button><br/>
      {/each}
    {:else if $config !== null}
      <p>
        Players:
        {#each $users as u, index}
          {#if index > 0}, {/if}
            <span class="user" class:self={u.id === $user_id} class:leader={u.leader}>{u.username}</span>
        {/each}
      </p>
      <p>Room is {$room_id}</p>
      {#if $user && $user.leader}
        <button on:click={() => client.start_game()}>
          Start game
        </button>
      {/if}
      <div id="config">
        <slot name="config" config={$config} readonly={!$user || !$user.leader} config_handler={e => config_handler(e)}></slot>
      </div>
    {:else if $view !== null}
      <div id="game">
        <slot name="game" do_action={e => do_action(e)}></slot>
      </div>
    {:else}
      <p>Something has gone wrong</p>
    {/if}
  {/if}
</main>

{#if admin_panel}
  <div class="admin-panel">
    <h1>Leader controls</h1>
    <button on:click={() => reset_to_lobby()}>Reset to lobby</button>
    {#each $users as u, index}
      <div>
        {u.username}
        {#if u.player_id !== null && reassigning === null}
          ({u.player_id})
          <button on:click={() => reassign_player(u.id)}>Reassign player</button>
        {/if}
        {#if reassigning !== null && u.player_id === null}
          <button on:click={() => reassign_to(u.id)}>Reassign to</button>
        {/if}
        <button on:click={() => kick_user(u.id)}>Kick</button>
      </div>
    {/each}
    {#if reassigning !== null}
      <button on:click={() => reassigning = null}>Cancel reassign</button>
    {/if}
  </div>
{/if}
{#if $user && $user.leader}
  <button class="admin-button" on:click={() => admin_panel = !admin_panel}>A</button>
{/if}

<style>
span.user {
}

span.user.self {
  font-weight: bold;
}

span.user.leader {
  color: #ffec00;
}

.admin-button {
  position: fixed;
  width: 30px;
  height: 30px;
  margin: 0px;
  padding: 0px;
  top: 5px;
  right: 5px;
}

.admin-panel {
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: #242424;
  padding: 50px;
  overflow-y: auto;
}
</style>
