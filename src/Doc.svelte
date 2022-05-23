<script lang="ts">
  import Loader from './Loader.svelte';
  import ParaList from './ParaList.svelte';

  let viewerElement: Element;
  let loader: Loader;
  export function getLoader() {
    return loader;
  }
</script>

<div class="viewer">
  <div class="content">
    <Loader
      bind:this={loader}
      component={ParaList}
      serverCommand={'get_paras'}
    />
  </div>
</div>

<style>
  .viewer {
    width: auto;
    height: inherit;
    box-sizing: border-box;
    overflow: scroll;
  }
  .content {
    box-sizing: border-box;
    padding: 0 var(--padding);
    width: 100%;
    height: auto;
  }
</style>
