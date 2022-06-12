<script lang="ts">
  import { onMount, tick } from 'svelte';

  export let items: any;
  export let serverCommand: (i: number, j: number) => Promise<any[]>;
  export let fetchAmount = 10;
  export let loaderHeight = 500;

  export let viewerElement: HTMLElement = null;
  let topLoaderElement: HTMLElement;
  let bottomLoaderElement: HTMLElement;
  let itemsElement: HTMLElement;

  let startIndex = 0;
  let endIndex = 0;
  let muteObserver = false;
  let topLoadShowing = true;
  let bottomLoadShowing = true;
  export function reset() {
    items = [];
    startIndex = 0;
    endIndex = 0;
    topLoadShowing = true;
    bottomLoadShowing = true;
  }
  export function getItemsElement() {
    return itemsElement;
  }

  let observer = new IntersectionObserver(
    (entries) => {
      if (muteObserver) return;
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
    if (!viewerElement) {
      viewerElement = topLoaderElement.parentElement.parentElement;
    }
    observer.observe(topLoaderElement);
    observer.observe(bottomLoaderElement);
    teleport(0);
  });
  async function loadItemsTop(amount: number) {
    let i = Math.max(0, startIndex - amount);
    let j = Math.max(0, startIndex);

    let newItems: any[] = await serverCommand(i, j);
    startIndex -= newItems.length;
    return newItems;
  }
  async function loadItemsBottom(amount: number) {
    let i = Math.max(0, endIndex);
    let j = Math.max(0, endIndex + amount);
    let newItems: any[] = await serverCommand(i, j);

    endIndex += newItems.length;
    return newItems;
  }
  async function extendItemsBottom() {
    if (
      bottomLoaderElement.offsetTop - viewerElement.scrollTop <
      viewerElement.clientHeight
    ) {
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
    await extendItemsTop();
  }
  async function loadBottom() {
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
    await extendItemsBottom();
  }
  // prevent teleports from being called at the same time
  // let teleportQueue = [];
  // export async function teleport(index: number, force?: boolean) {
  //   // if teleportQueue is empty, start a new chain
  //   if (teleportQueue.length == 0) {
  //     teleportChain(index, force);
  //   } else {
  //     // add args to queue
  //     let args = {
  //       index: index,
  //       force: force,
  //     };
  //     teleportQueue.push(args);
  //   }
  // }
  export async function teleport(index: number, force?: boolean) {
    muteObserver = true;
    if (!force && items.length > 0 && index >= startIndex && index < endIndex) {
      // if this forces it to load anything, just do normal teleport
      let item = itemsElement.children[index - startIndex] as HTMLElement;
      if (
        item &&
        !(item.offsetTop < loaderHeight) &&
        !(
          item.offsetTop + viewerElement.clientHeight >
          bottomLoaderElement.offsetTop
        )
      ) {
        viewerElement.scrollTop = item.offsetTop;
        return;
      }
    }
    reset();
    // loadbottom without observers noticing, so loadBottom doesn't get called twice
    startIndex = index;
    endIndex = index;
    await loadBottom();
    viewerElement.scrollTop = loaderHeight + 1;
    await tick();
    muteObserver = false;
  }
</script>

<div
  class="loader top"
  style={`height:${loaderHeight}px`}
  bind:this={topLoaderElement}
/>

<div class="items" bind:this={itemsElement}>
  <slot />
</div>

<div
  class="loader bottom"
  style={`height:${loaderHeight}px`}
  bind:this={bottomLoaderElement}
/>
<div class="spacefiller" style={`height:${viewerElement?.clientHeight}px`} />

<style>
  .loader {
    width: auto;
    /* border: 2px solid red; */
    box-sizing: border-box;
  }
</style>
