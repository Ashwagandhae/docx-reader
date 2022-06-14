<script>
  export let background = true;
  export let disabled = false;
</script>

<button
  on:click
  on:mousedown|preventDefault={() => {}}
  class:background
  {disabled}
>
  <slot />
</button>

<style>
  button {
    border: none;
    border-radius: var(--border-radius);
    cursor: pointer;
    line-height: 1rem;
    padding: 0.5rem;
    margin: 0;
    color: var(--text);
    height: 2rem;
    width: max-content;
    pointer-events: auto;
    display: block;
    background: none;
  }
  button[disabled] {
    pointer-events: none;
    opacity: 0.5;
  }
  button.background {
    background-color: var(--back-two);
  }
  button:hover {
    color: var(--text-strong);
  }
  button.background:hover {
    background-color: var(--back-two-hover);
  }
  button:active {
    background: none;
  }
  button.background:active {
    background-color: var(--back-two-active);
  }
</style>
