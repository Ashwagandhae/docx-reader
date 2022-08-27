<script lang="ts">
  import Button from './Button.svelte';
  import Search from './Search.svelte';
  import Icon from './Icon.svelte';
  import TurningArrow from './TurningArrow.svelte';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';
  import { listen } from '@tauri-apps/api/event';
  import { fade } from 'svelte/transition';
  import type { Query } from './types';

  export let showOutline: boolean;
  export let chooseFile: () => void;
  export let alignOutlineFocus: () => void;
  export let showSearchResults: boolean;
  export let matchCase: boolean;
  export let onlyOutline: boolean;

  let query: Writable<Query> = getContext('query');
  let zoom: Writable<number> = getContext('zoom');
  let isResizing: Writable<boolean> = getContext('isResizing');
  let fileInfo: Writable<{ open: boolean; name: string; path: string }> =
    getContext('fileInfo');

  let isFullscreen: Writable<boolean> = getContext('isFullscreen');
  // TODO fade out topbar when not needed
  // TODO stop link from being focused on page load
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
    {#if showOutline}
      <Button background={false} on:click={alignOutlineFocus}>
        <Icon name="link" />
      </Button>
    {/if}
    <Button on:click={() => (showOutline = !showOutline)} background={false}>
      <TurningArrow direction={showOutline ? 'left' : 'right'} />
    </Button>
  </section>
  <section class="general" class:open={$fileInfo.open} data-tauri-drag-region>
    <div class="start" data-tauri-drag-region>
      <Button on:click={() => ($zoom += 0.1)} background={false}
        ><Icon name="magnifyGlassPlus" /></Button
      >
      <Button on:click={() => ($zoom -= 0.1)} background={false}
        ><Icon name="magnifyGlassMinus" /></Button
      >
    </div>
    <div class="center" data-tauri-drag-region>
      <h1>
        {#if $fileInfo.open}
          {$fileInfo.name}
        {:else}
          No open file
        {/if}
      </h1>
      <Button on:click={chooseFile} hoverShadow>Open</Button>
    </div>
  </section>
  <section class="search" data-tauri-drag-region>
    <Search placeholder={'Search'} {matchCase} {onlyOutline} />
    <!-- todo make text not bleed -->
    {#if $query.text.length > 0}
      <div
        class="search-buttons"
        on:mousedown|preventDefault|stopPropagation={() => {}}
        transition:fade={{ duration: 300 }}
      >
        <Button
          on:click={() => (showSearchResults = !showSearchResults)}
          background={false}
        >
          <TurningArrow direction={showSearchResults ? 'right' : 'left'} />
        </Button>
      </div>
    {/if}
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
    gap: var(--padding-small);
    height: inherit;
    cursor: default;
    padding: 0 var(--padding-small);
    box-sizing: border-box;
  }
  .isFullscreen .outline {
    width: 40px;
  }
  .showOutline .outline,
  .search {
    width: var(--sidebar-width);
  }
  /* TODO make reactive to traffic lights when invis topbar */
  .outline {
    justify-content: flex-end;
    /* width: 100px; */
    width: 40px;
    transition: width 300ms;
  }

  .isResizing .outline {
    transition: none;
  }
  section.general {
    flex: 1;
    min-width: 0;
    align-items: center;
    justify-content: center;
    width: 100%;
  }
  section.general > .start {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }
  section.general > .center {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    height: inherit;
    padding: var(--padding-small);
    box-sizing: border-box;
    width: 100%;
    gap: var(--padding);
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
  .search {
    min-width: 0;
  }
  .search-buttons {
    position: absolute;
    right: var(--padding-small);
    border-radius: var(--border-radius);
  }
</style>
