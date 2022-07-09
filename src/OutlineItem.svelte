<script lang="ts">
  import type { RunType } from './types';
  import { getContext } from 'svelte';
  import type Loader from './Loader.svelte';
  import Button from './Button.svelte';
  import TurningArrow from './TurningArrow.svelte';

  export let outlineLevel: number = 0;
  $: indent = Math.min(outlineLevel, 3);
  export let toggleFold: () => void;
  export let link: number;
  export let runs: RunType[];
  export let index: number;
  export let selected: boolean = false;
  export let foldedParent: boolean = false;
  let text = '';
  $: {
    text = '';
    for (let run of runs) {
      text += run.text;
    }
  }
  let getDocLoader: () => Loader = getContext('getDocLoader');
</script>

<li
  style={`margin-left: ${indent}em`}
  on:click={() => {
    getDocLoader().teleport(link);
  }}
  class:selected
  class:foldedParent
  class:hasButtons={outlineLevel < 3}
>
  {#if outlineLevel < 3}
    <div class="buttons">
      <Button
        on:click={(e) => {
          e.stopPropagation();
          toggleFold();
        }}
        small={true}
        background={false}
      >
        <TurningArrow direction={foldedParent ? 'right' : 'down'} />
      </Button>
    </div>
  {/if}

  <div class="content">
    <span class:bold={indent < 3} class:big={indent < 2}>
      {text}
    </span>
  </div>
</li>

<style>
  li {
    display: flex;
    flex-direction: row;
    list-style-type: none;
    border-radius: var(--border-radius);
    cursor: default;
    min-height: 1em;
    font-size: 0.8em;
  }
  li:hover {
    background-color: var(--back-two-hover);
  }
  li:active,
  li.selected {
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
  /* TODO find a better solution for outline item fold buttons */
  .content {
    padding: var(--padding);
    padding-left: calc(var(--padding) * 2);
  }
  .hasButtons .content {
    padding-left: 0;
  }
  .buttons {
    opacity: 0;
  }
  li:hover .buttons,
  li.foldedParent .buttons {
    opacity: 1;
  }
</style>
