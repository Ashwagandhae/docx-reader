<script lang="ts">
  import type { StyleType } from './types';
  export let text: string;
  export let style: StyleType;
</script>

<span
  class:bold={style.bold}
  class:underline={style.underline}
  class:highlight={style.highlight}
  style={`font-size: ${style.size ? style.size * 0.5 : 10}pt;`}
>
  {text}
</span>

<style>
  span {
    overflow-wrap: break-word;
    color: var(--text);
    line-height: 1.5em;
  }
  .bold {
    font-weight: bold;
    color: var(--text-strong);
  }
  .underline {
    text-decoration: underline;
  }
  .highlight {
    background-color: var(--back-highlight);
    color: var(--text-strong);
  }
</style>
