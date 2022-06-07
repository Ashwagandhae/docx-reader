<script lang="ts">
  import type { RunType } from './types';
  import { getContext } from 'svelte';

  export let outline_level: number = 0;
  $: indent = Math.min(outline_level, 3);
  export let index: number;
  export let runs: RunType[];
  let text = '';
  $: {
    text = '';
    for (let run of runs) {
      text += run.text;
    }
  }
  let teleport: (index: number) => void = getContext('teleport');
</script>

<li style={`margin-left: ${indent}em`} on:click={() => teleport(index)}>
  <span class:bold={indent < 3} class:big={indent < 2}>
    {text}
  </span>
</li>

<style>
  li {
    list-style-type: none;
    padding: 0.5em;
    border-radius: var(--border-radius);
    cursor: default;
    min-height: 1em;
    font-size: 0.8em;
  }
  li:hover {
    background-color: var(--back-two-hover);
  }
  li:active {
    background-color: var(--back-two-active);
  }
  span {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    width: auto;
    overflow-wrap: break-word;
    word-break: break-word;
    overflow: hidden;
  }
  .bold {
    font-weight: var(--bold);
    color: var(--text-strong);
  }
  .big {
    font-size: 1.5em;
  }
</style>
