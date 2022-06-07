<script lang="ts">
  import Loader from './Loader.svelte';
  import Para from './Para.svelte';
  import { invoke } from '@tauri-apps/api';
  import { tick, onMount } from 'svelte';
  import type { ParaType } from './types';

  let viewerElement: HTMLElement;
  let loader: Loader;
  export async function teleport(index: number) {
    // if charindex isnt null
    loader?.teleport(index);
  }
  export function reset() {
    loader?.reset();
  }
  let items = [];
  async function serverCommand(i: number, j: number) {
    return (await invoke('get_paras', {
      i,
      j,
    })) as (ParaType & { charIndex?: number })[];
  }
  let zoom = 1;
  async function handleZoom(event: WheelEvent) {
    if (event.ctrlKey) {
      event.preventDefault();
      zoom -= event.deltaY * 0.01;
      zoom = Math.max(Math.min(zoom, 8), 0.3);
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
          let ratio =
            (viewerElement.scrollTop + viewerElement.clientHeight / 2) /
            (viewerElement.scrollHeight + diff);
          viewerElement.scrollTop =
            viewerElement.scrollHeight * ratio - viewerElement.clientHeight / 2;
        }
        lastSize = size;
      }
    }
  });
  function handleKeyDown(e: KeyboardEvent) {
    if (e.metaKey) {
      if (e.key === '=') {
        zoom += 0.1;
        zoom = Math.max(Math.min(zoom, 8), 0.3);
        let ratio =
          (viewerElement.scrollTop + viewerElement.clientHeight / 2) /
          viewerElement.scrollHeight;
        viewerElement.scrollTop =
          viewerElement.scrollHeight * ratio - viewerElement.clientHeight / 2;
      } else if (e.key === '-') {
        zoom -= 0.1;
        zoom = Math.max(Math.min(zoom, 8), 0.3);
        let ratio =
          (viewerElement.scrollTop + viewerElement.clientHeight / 2) /
          viewerElement.scrollHeight;
        viewerElement.scrollTop =
          viewerElement.scrollHeight * ratio - viewerElement.clientHeight / 2;
      }
    }
  }

  onMount(() => {
    resizeObserver.observe(parasElement);
  });
</script>

<svelte:window on:keydown={handleKeyDown} />
<div class="topbar" />
<div class="viewer" on:wheel={handleZoom} bind:this={viewerElement}>
  <div class="content" style={`font-size: ${zoom * 16}px;`}>
    <div class="paras-container">
      <div class="paras" bind:this={parasElement}>
        <Loader bind:this={loader} bind:items {viewerElement} {serverCommand}>
          {#each items as item (item)}
            <Para {...item} />
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
    padding: 0 0 0 calc(var(--sidebar-width));
    height: auto;
  }
  .paras-container {
    width: min(calc(100vw - var(--sidebar-width)));
    height: auto;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .paras {
    padding: var(--padding);
    box-sizing: border-box;
    width: min(100%, 55em);
  }
</style>
