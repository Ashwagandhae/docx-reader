<script lang="ts">
  import Icon from './Icon.svelte';
  import Button from './Button.svelte';
  import { Align } from './types';
  export let icon = 'ellipses';
  export let align: Align = Align.Left;
  export let title = 'Panel';
  let open = false;
  // TODO add panel animations
</script>

<div class="top" class:open>
  <!-- TODO make shadows connect  -->
  <div class="opener" class:right={align == Align.Right}>
    <Button on:click={() => (open = !open)} hoverShadow={true}>
      <Icon name={icon} />
    </Button>
  </div>
  {#if open}
    <div class="content">
      <h1>{title}</h1>
      <slot />
    </div>
  {/if}
</div>

<style>
  .top {
    width: 100%;
    height: min-content;
    position: relative;
    transition: height 1s;
    font-size: 0.8em;
  }
  .content {
    border-radius: var(--border-radius);
    box-shadow: var(--shadow);
    background: var(--back-two);
    padding: var(--padding);
  }
  .opener {
    position: absolute;
  }
  .opener.right {
    right: 0;
  }
  h1 {
    margin: 0;
    font-size: 1em;
    color: var(--text-strong);
    height: calc(2rem- var(--padding));
    padding-bottom: var(--padding);
  }
</style>
