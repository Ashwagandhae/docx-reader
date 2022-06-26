<script lang="ts">
  import Loader from './Loader.svelte';
  import OutlineItem from './OutlineItem.svelte';
  import type { ParaType } from './types';
  import { invoke } from '@tauri-apps/api';
  import { outlineAside } from './transition';
  import { getContext, onMount, tick } from 'svelte';
  import type { Writable } from 'svelte/store';

  let getDocLoader: () => Loader = getContext('getDocLoader');

  let docFocus: Writable<ParaType>;
  onMount(async function () {
    // wait until Doc is created
    await tick();
    docFocus = getDocLoader().getFocusStore();
  });
  let items = [];
  let selectedOutline = null;
  function docFocusChange() {
    for (let i = 0; i < items.length; i++) {
      // find the one after selectedOutline
      if (items[i].link > $docFocus.index) {
        selectedOutline = items[i].index - 1;
        return;
      } else if (items[i].link == $docFocus.index) {
        selectedOutline = items[i].index;
        return;
      }
    }
  }
  $: $docFocus, docFocusChange();

  let viewerElement: Element;
  let loader: Loader;
  export let showOutline: boolean;
  export function getLoader() {
    return loader;
  }
  async function serverCommand(i: number, j: number) {
    return (await invoke('get_outline_paras', {
      i,
      j,
    })) as ParaType[];
  }
</script>

{#if showOutline}
  <div class="top" transition:outlineAside>
    <div bind:this={viewerElement} class="viewer">
      <div class="content">
        <Loader bind:this={loader} bind:items {serverCommand} fetchAmount={30}>
          {#each items as item (item.index)}
            <OutlineItem
              outlineLevel={item.outline_level}
              link={item.link}
              runs={item.runs}
              selected={item.index === selectedOutline}
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
