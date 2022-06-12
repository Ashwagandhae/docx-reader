<script lang="ts">
  import { selectMark } from './transition';

  export let selected = false;
</script>

{#if selected}
  <mark in:selectMark class:selected>
    <slot />
  </mark>
{:else}
  <mark>
    <slot />
  </mark>
{/if}

<style>
  mark {
    font-size: max(15px, 1em);
    background-color: var(--back-mark);
    color: var(--text-strong);
    border-radius: 0.3em;
  }
  mark.selected {
    background-color: var(--back-mark-selected);
  }
</style>
