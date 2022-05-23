<script>
</script>

<button on:click>
  <slot />
</button>

<style>
  button {
    background-color: var(--back-two);
    border: none;
    border-radius: var(--border-radius);
    cursor: pointer;
    padding: 0.5em;
    margin: 0;
    color: var(--text);
  }
  button:hover {
    background-color: var(--back-two-hover);
    color: var(--text-strong);
  }
  button:active {
    background-color: var(--back-two-active);
  }
</style>
