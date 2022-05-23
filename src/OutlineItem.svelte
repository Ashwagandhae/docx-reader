<script lang="ts">
  import type { ParaType } from './types';
  import { messenger } from './stores';

  export let level: number = 0;
  $: indent = Math.min(level, 3);
  export let para: ParaType;
  let text = '';
  $: {
    text = '';
    for (let run of para.runs) {
      text += run.text;
    }
  }
  export let index: number;
  function teleport() {
    messenger.emit('teleport', index);
  }
</script>

<li style={`margin-left: ${indent}em`} on:click={teleport}>
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
    overflow: hidden;
  }
  .bold {
    font-weight: bold;
    color: var(--text-strong);
  }
  .big {
    font-size: 1.2em;
  }
</style>
