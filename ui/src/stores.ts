import { writable, derived } from 'svelte/store';

export const view = writable(null);
export const config = writable(null);
export const user_id = writable(null);
export const room_id = writable(null);
export const users = writable([]);

export const user_map = derived(
  users,
  $users => {
    let user_map = {};
    for (const user of $users) {
      user_map[user.id] = user;
    }
    return user_map;
  }, {});

export const player_map = derived(
  users,
  $users => {
    let player_map = {};
    for (const user of $users) {
      if (user.player_id !== null) {
        player_map[user.player_id] = user;
      }
    }
    return player_map;
  }, {});

export const user = derived(
  [user_map, user_id],
  ([$user_map, $user_id]) => $user_map[$user_id],
  null);

export const player_id = derived(
  user,
  $user => {
    if ($user) {
      return $user.player_id;
    }
    return null;
  }, null);
