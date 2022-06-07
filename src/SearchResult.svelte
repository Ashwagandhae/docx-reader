<script lang="ts">
  import type { ParaType } from './types';

  import { getContext, onMount } from 'svelte';
  import type { Writable } from 'svelte/store';

  export let link: number;
  export let index: number;
  export let text: string;
  export let queryIndex: number;
  export let selected: boolean;

  let query: Writable<string> = getContext('query');
  let selectedQuery: Writable<{ paraIndex: number; charIndex: number }> =
    getContext('selectedQuery');
  let startIndex: number;
  let charIndex: number;
  $: {
    // will be zero because +1
    charIndex = -1;
    let lowerText = text.toLowerCase();
    // get the specific index charIndex from queryIndex
    for (let i = 0; i < queryIndex + 1; i++) {
      charIndex = lowerText.indexOf($query, charIndex + 1);
    }
    startIndex = Math.max(0, charIndex - 30);
    while (text[startIndex - 1] !== ' ') {
      if (startIndex >= charIndex) {
        startIndex = Math.max(0, charIndex - 30);
        break;
      }
      startIndex++;
    }
  }
  let element: HTMLElement;
  function doTeleport() {
    teleport(link);
    selectedQuery.set({ paraIndex: link, charIndex: charIndex });
  }
  $: {
    if (selected) {
      doTeleport();
    }
  }
  export let selectSelf: () => void;
  function handleClick() {
    if (selected) {
      doTeleport();
    } else {
      selectSelf();
    }
  }

  let teleport: (index: number) => void = getContext('teleport');
</script>

<li
  on:click={handleClick}
  on:mousedown|preventDefault
  class:selected
  bind:this={element}
>
  <p>
    <span class:ellipses={startIndex != 0}
      >{text.slice(startIndex, charIndex)}</span
    ><mark class:selected
      >{text.slice(charIndex, charIndex + $query.length)}</mark
    ><span>{text.slice(charIndex + $query.length)}</span>
  </p>
</li>

<style>
  li {
    list-style-type: none;
    padding: 0.5em;
    display: block;
    position: relative;
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
  li.selected,
  li.selected:hover {
    background-color: var(--back-two-active);
  }
  p {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    width: auto;
    overflow-wrap: break-word;
    word-break: break-word;
    overflow: hidden;
    margin: 0;
  }
  span {
    color: var(--text);
  }
  span.ellipses::before {
    content: '...';
  }
</style>
