<script lang="ts">
  import Loader from './Loader.svelte';
  import OutlineItemList from './OutlineItemList.svelte';

  let viewerElement: Element;
  let loader: Loader;
  export function getLoader() {
    return loader;
  }
</script>

<div bind:this={viewerElement} class="viewer">
  <div class="content">
    <Loader
      bind:this={loader}
      component={OutlineItemList}
      serverCommand={'get_outline_items'}
      fetchAmount={30}
    />
  </div>
</div>

<style>
  .viewer {
    width: auto;
    height: inherit;
    box-sizing: border-box;
    overflow: scroll;
    padding: var(--padding);
    background-color: var(--back-two);
  }
  .content {
    box-sizing: border-box;
    padding: 0;
    width: 100%;
    height: auto;
  }
</style>
