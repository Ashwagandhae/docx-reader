<script lang="ts">
  import Button from './Button.svelte';
  import Search from './Search.svelte';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';
  import { listen } from '@tauri-apps/api/event';

  export let showOutline: boolean;
  export let chooseFile: () => void;
  export let showSearchResults: boolean;

  let isResizing: Writable<boolean> = getContext('isResizing');
  let fileInfo: Writable<{ open: boolean; name: string; path: string }> =
    getContext('fileInfo');

  let isFullscreen: Writable<boolean> = getContext('isFullscreen');
</script>

<div
  class="top"
  class:showOutline
  class:showSearchResults
  class:isResizing={$isResizing}
  class:isFullscreen={$isFullscreen}
  on:select={() => false}
  data-tauri-drag-region
>
  <section class="outline" data-tauri-drag-region>
    <Button on:click={() => (showOutline = !showOutline)}>
      {#if showOutline}
        &larr;
      {:else}
        &rarr;
      {/if}
    </Button>
  </section>
  <section class="middle" class:open={$fileInfo.open} data-tauri-drag-region>
    <h1>
      {#if $fileInfo.open}
        {$fileInfo.name}
      {:else}
        No open file
      {/if}
    </h1>
    <Button on:click={chooseFile}>Open</Button>
  </section>
  <section class="search" data-tauri-drag-region>
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

    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }

  .top > section {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: var(--padding);
    height: inherit;
    cursor: default;
    padding: 0 calc(var(--padding) / 2);
    box-sizing: border-box;
  }
  .isFullscreen .outline {
    width: 40px;
  }
  .showOutline .outline,
  .search {
    width: var(--sidebar-width);
  }
  /* todo make width not fixed */
  .outline {
    justify-content: flex-end;
    width: 100px;
    transition: width 300ms;
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
