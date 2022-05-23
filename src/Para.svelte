<script lang="ts">
  import type { RunType } from './types';
  import Run from './Run.svelte';
  export let runs: RunType[] = [];
  export let outline_level: number = 10;
</script>

<p>
  {#each runs as run}
    <Run text={run.text} style={run.style} />
  {/each}
</p>

<style>
  p {
    overflow-wrap: break-word;
    word-break: break-all;
    margin: 0;
    padding: 1em 0;
  }
</style>
