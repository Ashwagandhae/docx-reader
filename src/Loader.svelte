<script lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { onMount, tick } from 'svelte';

  export let component: any;
  export let serverCommand: string;
  export let fetchAmount = 10;
  export let loaderHeight = 500;

  let viewerElement: HTMLElement;
  let topLoaderElement: HTMLElement;
  let bottomLoaderElement: HTMLElement;
  let itemsElement: HTMLElement;

  let items = [];
  let startIndex = 0;
  let endIndex = 0;
  let topLoadShowing = true;
  let bottomLoadShowing = true;
  export function reset() {
    items = [];
    startIndex = 0;
    endIndex = 0;
    topLoadShowing = true;
    bottomLoadShowing = true;
  }

  let observer = new IntersectionObserver(
    (entries) => {
      let shouldLoadBottom = false;
      let shouldLoadTop = false;
      for (let entry of entries) {
        if (entry.isIntersecting) {
          shouldLoadBottom = entry.target == bottomLoaderElement;
          shouldLoadTop = entry.target == topLoaderElement;
        }
      }
      // prioritize loadBottom first
      if (shouldLoadBottom) {
        loadBottom();
      } else if (shouldLoadTop) {
        loadTop();
      }
      topLoadShowing = shouldLoadTop;
      bottomLoadShowing = shouldLoadBottom;
    },
    {
      root: viewerElement,
      rootMargin: '0px',
      threshold: 0,
    }
  );

  onMount(async function () {
    viewerElement = topLoaderElement.parentElement.parentElement;
    observer.observe(topLoaderElement);
    observer.observe(bottomLoaderElement);
    reset();
  });
  async function loadItemsTop(amount: number) {
    let i = Math.max(0, startIndex - amount);
    let j = Math.max(0, startIndex);

    let newItems: any[] = await invoke(serverCommand, { i, j });
    startIndex -= newItems.length;
    return newItems;
  }
  async function loadItemsBottom(amount: number) {
    let i = Math.max(0, endIndex);
    let j = Math.max(0, endIndex + amount);
    let newItems: any[] = await invoke(serverCommand, { i, j });
    endIndex += newItems.length;
    return newItems;
  }
  async function extendItemsBottom() {
    if (
      bottomLoaderElement.offsetTop - viewerElement.scrollTop <
      viewerElement.clientHeight
    ) {
      console.log('extend bottom');
      let newItems = await loadItemsBottom(fetchAmount);
      if (newItems.length == 0) return;
      items = [...items, ...newItems];
      await tick();
      extendItemsBottom();
    }
  }
  async function extendItemsTop() {
    if (
      topLoaderElement.offsetTop -
        viewerElement.scrollTop +
        topLoaderElement.clientHeight >
      0
    ) {
      console.log('extend top');
      let newItems = await loadItemsTop(fetchAmount);
      if (newItems.length == 0) return;
      let oldHeight = itemsElement.clientHeight;
      items = [...items, ...newItems];
      await tick();
      viewerElement.scrollTop += itemsElement.clientHeight - oldHeight;
      extendItemsTop();
    }
  }
  async function loadTop() {
    let newItems = await loadItemsTop(fetchAmount);
    if (newItems.length == 0) return;
    // remove out of view items
    let itemsHeightChange = 0;
    for (let i = itemsElement.children.length - 1; i >= 0; i--) {
      let child = itemsElement.children[i] as HTMLElement;
      let childTop = child.offsetTop - viewerElement.scrollTop;
      if (childTop > viewerElement.clientHeight) {
        items.pop();
        endIndex -= 1;
        itemsHeightChange -= child.clientHeight;
      } else {
        break;
      }
    }
    let oldHeight = itemsElement.clientHeight;
    items = [...newItems, ...items];
    await tick();
    itemsHeightChange += oldHeight - itemsElement.clientHeight;
    viewerElement.scrollTop -= itemsHeightChange;
    extendItemsTop();
  }
  export async function loadBottom() {
    let newItems = await loadItemsBottom(fetchAmount);
    if (newItems.length == 0) return;
    // remove out of view items
    let itemsHeightChange = 0;
    for (let i = 0; i < itemsElement.children.length; i++) {
      let child = itemsElement.children[i] as HTMLElement;
      // if child is outside of viewerElement
      let childBottom =
        child.offsetTop - viewerElement.scrollTop + child.clientHeight;

      if (childBottom < 0) {
        items.shift();
        startIndex += 1;
        itemsHeightChange -= child.clientHeight;
      } else {
        break;
      }
    }
    items = [...items, ...newItems];
    await tick();
    viewerElement.scrollTop += itemsHeightChange;
    extendItemsBottom();
  }
  export async function teleport(index: number) {
    reset();
    startIndex = index;
    endIndex = index;
    await loadBottom();
    viewerElement.scrollTop = loaderHeight + 1;
  }
</script>

<div
  class="loader top"
  style={`height:${loaderHeight}px`}
  bind:this={topLoaderElement}
/>

<div class="items" bind:this={itemsElement}>
  <svelte:component this={component} {items} />
</div>

<div
  class="loader bottom"
  style={`height:${loaderHeight}px`}
  bind:this={bottomLoaderElement}
/>

<style>
  .loader {
    width: auto;
    /* border: 2px solid red; */
    box-sizing: border-box;
  }
</style>
