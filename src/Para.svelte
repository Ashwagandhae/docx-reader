<script lang="ts">
  import type { RunType } from './types';
  import { getContext, tick } from 'svelte';
  import { writable } from 'svelte/store';
  import type { Writable } from 'svelte/store';

  import Run from './Run.svelte';
  import { run } from 'svelte/internal';
  export let runs: RunType[] = [];
  export let outline_level: number;
  export let index: number;

  let query: Writable<string> = getContext('query');
  let selectedQuery: Writable<{ paraIndex: number; charIndex: number }> =
    getContext('selectedQuery');
  let displayRuns: (RunType & {
    queryMatches?: number[];
    selectedQueryMatch?: number;
  })[] = [];
  let combinedText = runs.reduce((prev, curr) => prev + curr.text, '');
  function onQueryUpdate() {
    displayRuns = [];

    let matches = [];
    if ($query.length > 0) {
      matches = [...combinedText.matchAll(new RegExp($query, 'gi'))].map(
        (a) => a.index
      );
    }
    let i = 0;
    for (let run of runs) {
      let queryMatches = [];
      let selectedQueryMatch = undefined;
      for (let match of matches) {
        if (match + $query.length >= i && match < i + run.text.length) {
          queryMatches.push(match - i);
          if (
            index === $selectedQuery.paraIndex &&
            match === $selectedQuery.charIndex
          ) {
            selectedQueryMatch = queryMatches.length - 1;
          }
        }
      }
      displayRuns.push({
        ...run,
        queryMatches,
        selectedQueryMatch,
      });

      i += run.text.length;
    }
  }
  $: $query, $selectedQuery, onQueryUpdate();
</script>

<p>
  {#each displayRuns as run}
    <Run
      text={run.text}
      style={run.style}
      queryMatches={run.queryMatches}
      selectedQueryMatch={run.selectedQueryMatch}
    />
  {/each}
</p>

<style>
  p {
    overflow-wrap: break-word;
    word-break: break-word;
    margin: 0;
    display: block;
    padding: 0;
    padding-top: 2em;
  }
</style>
