import { derived } from 'svelte/store';
import { view } from './stores.js';

export const row_labels = derived(
  view,
  $view => {
    if ($view !== null) {
      return $view.labels.slice(0, $view.size.row);
    } else {
      return [];
    }
  });

export const col_labels = derived(
  view,
  $view => {
    if ($view !== null) {
      return $view.labels.slice($view.size.row);
    } else {
      return [];
    }
  });
