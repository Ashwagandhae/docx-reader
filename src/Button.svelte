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
    line-height: 1em;
    padding: 0.5em;
    margin: 0;
    color: var(--text);
    height: 2em;
    pointer-events: auto;
    display: block;
  }
  button:hover {
    background-color: var(--back-two-hover);
    color: var(--text-strong);
  }
  button:active {
    background-color: var(--back-two-active);
  }
</style>
