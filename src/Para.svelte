<script lang="ts">
  import type { RunType } from './types';
  import { getContext, onMount, tick } from 'svelte';
  import { writable } from 'svelte/store';
  import type { Writable } from 'svelte/store';
  import { paraButtons } from './transition';

  import Run from './Run.svelte';
  import Icon from './Icon.svelte';
  import Button from './Button.svelte';
  export let copySelfAndChildren: () => Promise<unknown>;
  export let runs: RunType[] = [];
  export let outline_level: number;
  export let index: number;
  export let viewerElement: HTMLElement;
  let elementType = 'p';
  if (outline_level === 0) {
    elementType = 'h1';
  } else if (outline_level === 1) {
    elementType = 'h2';
  } else if (outline_level === 2) {
    elementType = 'h3';
  } else if (outline_level === 3) {
    elementType = 'h4';
  } else if (outline_level === 4) {
    elementType = 'h5';
  } else if (outline_level === 5) {
    elementType = 'h6';
  }

  let query: Writable<string> = getContext('query');
  let selectedQuery: Writable<{ paraIndex: number; charIndex: number }> =
    getContext('selectedQuery');
  let displayRuns: (RunType & {
    queryMatches?: number[];
    selectedQueryMatch?: number;
  })[] = [];
  let combinedText = runs.reduce((prev, curr) => prev + curr.text, '');
  function onQueryUpdate() {
    displayRuns = [];

    let matches = [];
    if ($query.length > 0) {
      matches = [...combinedText.matchAll(new RegExp($query, 'gi'))].map(
        (a) => a.index
      );
    }
    let i = 0;
    for (let run of runs) {
      let queryMatches = [];
      let selectedQueryMatch = undefined;
      for (let match of matches) {
        if (match + $query.length >= i && match < i + run.text.length) {
          queryMatches.push(match - i);
          if (
            index === $selectedQuery.paraIndex &&
            match === $selectedQuery.charIndex
          ) {
            selectedQueryMatch = queryMatches.length - 1;
          }
        }
      }
      displayRuns.push({
        ...run,
        queryMatches,
        selectedQueryMatch,
      });

      i += run.text.length;
    }
  }
  $: $query, $selectedQuery, onQueryUpdate();
  function getClipboardHTML() {
    let paraNode = document.createElement(elementType);
    paraNode.setAttribute(
      'style',
      `
      font-family: Calibri;
      line-height: 1em;
    `
    );
    for (let run of runs) {
      let runNode = document.createElement('span');
      runNode.setAttribute(
        'style',
        `
        font-weight: ${run.style.bold ? 'bold' : 'normal'};
        text-decoration: ${run.style.underline ? 'underline' : 'none'};
        font-size: ${run.style.size ? run.style.size / 2 : 12}pt;
        background-color: ${run.style.highlight ? 'yellow' : 'none'};
      `
      );
      runNode.innerText = run.text.replaceAll('\n', '').replaceAll('\r', '');
      paraNode.appendChild(runNode);
    }
    return paraNode;
  }
  function getClipboardText() {
    let text = '';
    for (let run of runs) {
      text += run.text;
    }
    return text;
  }
  function copySelf() {
    let clipboardHTML = getClipboardHTML();
    const blob = new Blob([clipboardHTML.outerHTML], { type: 'text/html' });
    const clipboardItem = new window.ClipboardItem({ 'text/html': blob });
    navigator.clipboard.write([clipboardItem]);
  }
  let loading = false;
</script>

<div class="top">
  <div class="buttons-container">
    <div class="buttons" class:loading>
      <Button
        on:click={() => {
          copySelf();
        }}
      >
        <Icon name="copy" />
      </Button>
      <Button
        disabled={loading}
        on:click={async () => {
          loading = true;
          await copySelfAndChildren();
          loading = false;
        }}
      >
        <Icon name="copy" />
      </Button>
    </div>
  </div>
  <svelte:element this={elementType} class="para">
    {#each displayRuns as run}
      <Run
        text={run.text}
        style={run.style}
        queryMatches={run.queryMatches}
        selectedQueryMatch={run.selectedQueryMatch}
      />
    {/each}
  </svelte:element>
</div>

<style>
  .top {
    position: relative;
    display: grid;
    grid-template-columns: 3rem auto;
    grid-template-rows: auto;
    justify-content: left;
    gap: var(--padding);
    height: auto;
    width: auto;
    transition: background 0.3s;
  }
  .buttons-container {
    position: relative;
    width: 2rem;
    height: auto;
  }
  .buttons {
    padding-top: 10%;

    position: absolute;
    display: flex;
    flex-direction: column;
    gap: var(--padding);
    opacity: 0;
    color: var(--text);
    transition: opacity var(--transition-speed);
  }

  .buttons.loading,
  .top:hover .buttons {
    opacity: 1;
  }
  .buttons.loading {
    color: var(--text-weak);
  }
  .para {
    font-size: 1em;
    text-decoration: none;
    font-weight: normal;
    display: block;
    position: relative;
    overflow-wrap: break-word;
    word-break: break-word;
    margin: 0;
    padding: 0;
    padding-bottom: 2em;
  }
</style>
