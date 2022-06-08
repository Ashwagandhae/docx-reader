<script lang="ts">
  import Loader from './Loader.svelte';
  import SearchResult from './SearchResult.svelte';
  import Search from './Search.svelte';
  import { invoke } from '@tauri-apps/api';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';
  import type { SearchResultType } from './types';
  import { tick } from 'svelte';
  import { searchAside } from './transition';

  let viewerElement: HTMLElement;
  let loader: Loader;
  let query: Writable<string> = getContext('query');

  export function teleport(index: number) {
    loader?.teleport(index);
  }
  export function reset() {
    loader?.reset();
  }
  async function serverCommand(i: number, j: number) {
    if ($query.length > 0) {
      let ret = (await invoke('search', {
        query: $query,
        i: i,
        j: j,
      })) as SearchResultType[];
      return ret;
    }
    return [];
  }
  export let showSearchResults: boolean;
  $: $query, onQueryUpdate();
  async function onQueryUpdate() {
    selectedResultIndex = -1;
    if ($query.length > 0) {
      // only call teleport if loader already exists (it teleports by itself onMount)
      loader && loader.teleport(0, true);
    }
  }
  let selectedResultIndex: number = -1;
  function select(index: number) {
    selectedResultIndex = index;
  }
  let items = [];
  async function indexResult(newIndex: number) {
    // if element is loaded
    if (
      newIndex >= items[0].index &&
      newIndex < items[items.length - 1].index
    ) {
      let element = loader.getItemsElement().children[
        newIndex - items[0].index
      ] as HTMLElement;
      // if its out of view
      if (
        element &&
        (element.offsetTop < viewerElement.scrollTop ||
          element.offsetTop + element.offsetHeight >
            viewerElement.scrollTop + viewerElement.clientHeight)
      ) {
        // teleport to it
        await loader.teleport(newIndex);
        select(newIndex);
      } else {
        // else just select it
        select(newIndex);
      }
    } else {
      // else teleport to it
      await loader.teleport(newIndex);
      // if that was the last item, select the one before it
      if (items.length == 0) {
        newIndex -= 1;
        await loader.teleport(newIndex);
      }
      select(newIndex);
    }
  }
  export function prevResult() {
    if (selectedResultIndex > 0) {
      indexResult(selectedResultIndex - 1);
    }
  }
  export function nextResult() {
    indexResult(selectedResultIndex + 1);
  }
</script>

{#if $query.length > 0 && showSearchResults}
  <div class="top" transition:searchAside>
    <div bind:this={viewerElement} class="viewer">
      <div class="content">
        <Loader
          bind:this={loader}
          bind:items
          {viewerElement}
          {serverCommand}
          fetchAmount={30}
        >
          {#each items as item}
            <SearchResult
              link={item.link}
              index={item.index}
              text={item.text}
              queryIndex={item.query_index}
              selected={item.index == selectedResultIndex}
              selectSelf={() => select(item.index)}
            />
          {/each}
        </Loader>
      </div>
    </div>
  </div>
{/if}

<style>
  .top {
    background-color: var(--back-two);
    width: 100%;
    height: 100vh;
    box-sizing: border-box;
    padding-top: var(--topbar-height);
  }
  .viewer {
    padding: var(--padding);
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    overflow: scroll;
    position: relative;
  }
  .content {
    box-sizing: border-box;
    padding: 0;
    width: 100%;
    height: auto;
  }
</style>
