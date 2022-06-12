<script lang="ts">
  import type { StyleType } from './types';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';
  import Mark from './Mark.svelte';
  let query: Writable<string> = getContext('query');
  export let text: string;
  export let style: StyleType;
  export let queryMatches: number[];
  export let selectedQueryMatch: number;
  let splitText: string[] = [];
  // 0 is nothing, 1 is normal match 2 is selectede match
  let matchesDict: number[] = [];
  // split text into parts that match and parts that dont

  function onQueryMatchesUpdate() {
    splitText = [];
    matchesDict = [];
    let lastMatchEnd = 0;
    queryMatches.forEach(function (match, index) {
      if (match - lastMatchEnd > 0) {
        splitText.push(text.substring(lastMatchEnd, match));
        matchesDict.push(0);
      }
      splitText.push(text.substring(match, match + $query.length));
      if (index === selectedQueryMatch) {
        matchesDict.push(2);
      } else {
        matchesDict.push(1);
      }
      lastMatchEnd = match + $query.length;
    });
    if (text.length - lastMatchEnd > 0) {
      splitText.push(text.substring(lastMatchEnd));
      matchesDict.push(0);
    }
    splitText = splitText;
    matchesDict = matchesDict;
  }
  $: queryMatches, selectedQueryMatch, onQueryMatchesUpdate();
</script>

<span
  class:bold={style.bold}
  class:underline={style.underline}
  class:highlight={style.highlight}
  style={`font-size: ${style.size ? style.size * 0.05 : 1}em;`}
>
  {#each splitText as part, index}
    {#if matchesDict[index] == 2}
      <Mark selected={true}>{part}</Mark>
    {:else if matchesDict[index] == 1}
      <Mark>{part}</Mark>
    {:else}
      {part}
    {/if}
  {/each}
</span>

<style>
  span {
    overflow-wrap: break-word;
    color: var(--text);
    line-height: 2em;
    display: inline;
    border-radius: 0.3em;
  }
  .bold {
    font-weight: var(--bold);
    color: var(--text-strong);
  }
  .underline {
    text-decoration: underline;
  }
  .highlight {
    background-color: var(--back-highlight);
    color: var(--text-strong);
  }
</style>
