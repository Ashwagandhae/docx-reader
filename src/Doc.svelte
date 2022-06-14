<script lang="ts">
  import Loader from './Loader.svelte';
  import Para from './Para.svelte';
  import { invoke } from '@tauri-apps/api';
  import { tick, onMount } from 'svelte';
  import type { ParaType } from './types';
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';
  import {
    getSelectionString,
    copyToClipboard,
    getParaHTML,
  } from './selection';

  let query: Writable<string> = getContext('query');
  let zoom: Writable<number> = getContext('zoom');

  export let showOutline: boolean;
  export let showSearchResults: boolean;

  let viewerElement: HTMLElement;
  let loader: Loader;
  export function getLoader() {
    return loader;
  }
  let items = [];
  async function serverCommand(i: number, j: number) {
    return (await invoke('get_paras', {
      i,
      j,
    })) as (ParaType & { charIndex?: number })[];
  }
  async function handle$zoom(event: WheelEvent) {
    if (event.ctrlKey) {
      event.preventDefault();
      $zoom -= event.deltaY * 0.01;
      $zoom = Math.max(Math.min($zoom, 8), 0.3);
      let ratio =
        (viewerElement.scrollTop + viewerElement.clientHeight / 2) /
        viewerElement.scrollHeight;
      await tick();
      viewerElement.scrollTop =
        viewerElement.scrollHeight * ratio - viewerElement.clientHeight / 2;
    }
  }
  let parasElement: HTMLElement;
  let lastSize: number[] = [undefined, undefined];
  const resizeObserver = new ResizeObserver((entries) => {
    for (let entry of entries) {
      if (entry.target === parasElement) {
        let size = [entry.contentRect.width, entry.contentRect.height];
        if (lastSize[0] !== size[0]) {
          let diff = lastSize[1] - size[1];
          let oldHeight = viewerElement.clientHeight;
          let ratio =
            (viewerElement.scrollTop + viewerElement.clientHeight / 2) /
            (viewerElement.scrollHeight + diff);
          viewerElement.scrollTop =
            viewerElement.scrollHeight * ratio - oldHeight / 2;
        }
        lastSize = size;
      }
    }
  });
  function handleKeyDown(e: KeyboardEvent) {
    if (e.metaKey) {
      if (e.key === '=') {
        e.preventDefault();
        $zoom += 0.1;
        $zoom = Math.max(Math.min($zoom, 8), 0.3);
        let ratio =
          (viewerElement.scrollTop + viewerElement.clientHeight / 2) /
          viewerElement.scrollHeight;
        viewerElement.scrollTop =
          viewerElement.scrollHeight * ratio - viewerElement.clientHeight / 2;
      } else if (e.key === '-') {
        e.preventDefault();

        $zoom -= 0.1;
        $zoom = Math.max(Math.min($zoom, 8), 0.3);
        let ratio =
          (viewerElement.scrollTop + viewerElement.clientHeight / 2) /
          viewerElement.scrollHeight;
        viewerElement.scrollTop =
          viewerElement.scrollHeight * ratio - viewerElement.clientHeight / 2;
      } else if (e.key === 'c') {
        e.preventDefault();
        copyToClipboard(getSelectionString(parasElement, items));
      }
    }
  }

  $: {
    if (showSearchResults && $query.length > 0 && !showOutline) {
      viewerElement.scrollLeft = viewerElement.scrollWidth;
    }
  }

  onMount(() => {
    resizeObserver.observe(parasElement);
  });
  async function copyParaAndChildren(index: number) {
    let ret = document.createElement('div');
    let loadedItems = items.slice(index);
    let para = items[index];
    ret.appendChild(getParaHTML(para));
    let i = 1;
    while (i < loadedItems.length) {
      let child = loadedItems[i];
      if (
        para.outline_level != null &&
        (child.outline_level == null ||
          child.outline_level > para.outline_level)
      ) {
        ret.appendChild(getParaHTML(child));
      } else {
        break;
      }
      // load more if needed
      if (i + 1 >= loadedItems.length) {
        let currentIndex = loadedItems[loadedItems.length - 1].index;
        let newItems = await serverCommand(
          currentIndex + 1,
          currentIndex + 1 + 30
        );
        // will be added by 1
        i = -1;
        loadedItems = newItems;
      }
      i++;
    }
    copyToClipboard(ret.innerHTML);
  }
</script>

<svelte:window on:keydown={handleKeyDown} />
<div
  class="viewer"
  on:wheel={handle$zoom}
  bind:this={viewerElement}
  class:showOutline
  class:showSearchResults={showSearchResults && $query.length > 0}
>
  <div class="content" style={`font-size: ${$zoom * 16}px;`}>
    <div class="paras-container">
      <div class="paras" bind:this={parasElement}>
        <Loader bind:this={loader} bind:items {viewerElement} {serverCommand}>
          {#each items as item, index (item.index)}
            <Para
              {...item}
              {viewerElement}
              copySelfAndChildren={() => copyParaAndChildren(index)}
            />
          {/each}
        </Loader>
      </div>
    </div>
  </div>
</div>

<style>
  .viewer {
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    overflow: scroll;
    padding: 0;
  }
  .content {
    font-size: 16px;
    box-sizing: content-box;
    width: 100vw;
    padding: 0;
    height: auto;
    /* transition: padding 300ms; */
  }

  .showSearchResults.showOutline .content {
    padding: 0 0 0 calc(var(--sidebar-width));
  }
  .paras-container {
    width: 100vw;
    height: auto;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .showOutline.showSearchResults .paras-container {
    width: min(calc(100vw - var(--sidebar-width)));
  }
  .paras-container {
    padding: 0;
  }
  .showOutline .paras-container {
    padding-left: var(--sidebar-width);
  }
  .showSearchResults .paras-container {
    padding-right: var(--sidebar-width);
  }
  .showOutline.showSearchResults .paras-container {
    padding: 0;
  }

  .paras {
    padding: var(--padding);
    box-sizing: border-box;
    width: min(100%, 50em);
  }
</style>
