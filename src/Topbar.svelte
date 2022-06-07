<script lang="ts">
  import Button from './Button.svelte';
  import Search from './Search.svelte';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';

  export let showOutline: boolean;
  export let chooseFile: () => void;
  export let showSearchResults: boolean;

  let isResizing: Writable<boolean> = getContext('isResizing');
  let fileInfo: Writable<{ open: boolean; name: string; path: string }> =
    getContext('fileInfo');
</script>

<div
  class="top"
  class:showOutline
  class:showSearchResults
  class:isResizing={$isResizing}
>
  <section class="outline">
    <Button on:click={() => (showOutline = !showOutline)}
      >{#if showOutline}
        &larr;
      {:else}
        &rarr;
      {/if}</Button
    >
  </section>
  <section class="middle" class:open={$fileInfo.open}>
    <h1>
      {#if $fileInfo.open}
        {$fileInfo.name}
      {:else}
        No open file
      {/if}
    </h1>
    <Button on:click={chooseFile}>Open</Button>
  </section>
  <section class="search">
    <Search placeholder={'Search'} />
  </section>
</div>

<style>
  .top {
    display: flex;
    flex-direction: row;
    align-items: center;
    width: inherit;
    height: inherit;
    gap: calc(var(--padding) / 2);
  }
  .top > section {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: var(--padding);
    height: inherit;
  }
  .showOutline .outline,
  .search {
    width: var(--sidebar-width);
  }
  /* todo make width not fixed */
  .outline {
    justify-content: flex-end;
    width: 40px;
    transition: width 300ms;
  }
  .outline,
  .search {
    box-sizing: border-box;
    padding: 0 calc(var(--padding) / 2);
  }
  .isResizing .outline {
    transition: none;
  }
  .middle {
    flex: 1;
    justify-content: center;
    min-width: 0;
    overflow: hidden;
  }
  .search {
    min-width: 0;
    overflow: hidden;
  }
  h1 {
    font-size: 1em;
    font-weight: 400;
    margin: 0;
    color: var(--text-weak);
    white-space: nowrap;
  }
  .open h1 {
    color: var(--text-strong);
    font-weight: var(--bold);
  }
</style>
