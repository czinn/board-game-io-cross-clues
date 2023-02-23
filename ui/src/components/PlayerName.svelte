<script>
import { player_map } from '../stores.js';

export let player_id;

$: player = $player_map && $player_map[player_id];
</script>

<span class="username">{player && player.username}</span>

<style>
.username {
  font-weight: bold;
}
</style>
