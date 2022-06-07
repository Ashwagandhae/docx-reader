<script lang="ts">
  import Loader from './Loader.svelte';
  import OutlineItem from './OutlineItem.svelte';
  import type { ParaType } from './types';
  import { invoke } from '@tauri-apps/api';
  import { outlineAside } from './transition';

  let viewerElement: Element;
  let loader: Loader;
  export let showOutline: boolean;
  export function teleport(index: number) {
    loader?.teleport(index);
  }
  export function reset() {
    loader?.reset();
  }
  let items = [];
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
          {#each items as item (item)}
            <OutlineItem {...item} />
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
