<script>
  import Para from './Para.svelte';

  export let items = [];
</script>

{#each items as item (item)}
  <Para {...item} />
{/each}
