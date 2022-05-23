<script>
  import OutlineItem from './OutlineItem.svelte';

  export let items = [];
</script>

{#each items as item (item)}
  <OutlineItem {...item} />
{/each}
