<script lang="ts">
  export let value: boolean = true;
  export let label: string = 'checkbox';
</script>

<label>
  <div class="labelText">
    {label}
  </div>
  <input bind:checked={value} type="checkbox" />
</label>

<style>
  label {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: start;
    padding: var(--padding);
    border-radius: var(--border-radius);
  }
  input {
    margin: 0;
    min-width: 0;
    flex-shrink: 0;
  }
</style>
