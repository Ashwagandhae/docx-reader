<script lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { fade } from 'svelte/transition';
  import { onMount, tick } from 'svelte';
  import Doc from './Doc.svelte';
  import Outline from './Outline.svelte';
  import Button from './Button.svelte';
  import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
  import { messenger } from './stores';
  import { listen } from '@tauri-apps/api/event';

  // listen for changes in system settings

  let colorThemeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  let updateColorTheme = function () {
    document.body.classList.toggle('dark', colorThemeMediaQuery.matches);
    alert(colorThemeMediaQuery.matches);
  };
  updateColorTheme();
  colorThemeMediaQuery.addEventListener('change', updateColorTheme);

  async function chooseFile() {
    let filepath: string = await invoke('open_dialog');
    if (!filepath) return;
    readFile(filepath);
  }
  let droppingFile = false;
  listen('tauri://file-drop-hover', () => {
    console.log('among');
    droppingFile = true;
  });
  listen('tauri://file-drop-cancelled', () => {
    droppingFile = false;
  });
  listen('tauri://file-drop', (event) => {
    droppingFile = false;
    if ((event.payload as string[]).length > 0) {
      readFile(event.payload[0]);
    }
  });
  async function readFile(filepath: string) {
    let extension = filepath.split('.').pop();
    if (extension != 'docx') return;
    await closeFile();
    let fileResult = await invoke('load_file', { filepath });
    if (!fileResult) return;
    doc.getLoader().teleport(0);
    outline.getLoader().teleport(0);
  }
  async function closeFile() {
    let result = await invoke('unload_file');
    outline.getLoader().reset();
    doc.getLoader().reset();
    await tick();
  }
  messenger.on('teleport', function (index) {
    doc.getLoader().teleport(index);
  });
  let doc: Doc;
  let outline: Outline;
</script>

<main>
  {#if droppingFile}
    <div class="screen" transition:fade={{ duration: 200 }}>
      <div class="message">drop file here</div>
    </div>
  {/if}
  <div class="grid">
    <div class="tools">
      <Button on:click={chooseFile}>Open file</Button>
      <Button on:click={closeFile}>Close file</Button>
    </div>
    <div class="doc">
      <Doc bind:this={doc} />
    </div>
    <div class="outline">
      <Outline bind:this={outline} />
    </div>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: sans-serif;
    color: var(--text);
    background-color: var(--back);
  }
  main {
    position: relative;
  }
  .screen {
    z-index: 10000;
    position: absolute;
    top: 0;
    left: 0;
    background-color: hsl(0, 0%, 0%, 0.5);
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .message {
    font-size: 1em;
    color: var(--text);
  }
  .tools {
    width: 100vw;
    display: flex;
    box-sizing: border-box;
    justify-content: center;
    align-items: center;
    position: absolute;
    background-color: var(--back-two);
    z-index: 9999;
    height: var(--topbar-height);
    gap: var(--padding);
    padding: 5px;
  }
  .tools > button {
    padding: 5px;
    margin: 0;
  }
  .grid {
    display: grid;
    grid-template-columns: max(200px, 20%) 1fr;
    grid-template-rows: var(--topbar-height) auto;
    grid-template-areas:
      'tools tools'
      'outline doc';
    gap: var(--gap);
    width: 100vw;
    height: 100vh;
  }
  .tools {
    grid-area: tools;
  }
  .doc {
    position: relative;
    height: calc(100vh - var(--topbar-height) - var(--gap));
    grid-area: doc;
  }
  .outline {
    position: relative;
    height: calc(100vh - var(--topbar-height) - var(--gap));
    grid-area: outline;
  }
  :global(body) {
    --padding: 10px;
    --topbar-height: 50px;
    --gap: 0px;
    --border-radius: 10px;
  }
  :global(body) {
    --back: hsl(0, 0%, 100%);
    --back-two: hsl(0, 0%, 90%);
    --back-highlight: hsl(195, 80%, 80%);

    --text: hsl(0, 0%, 25%);
    --text-strong: hsl(0, 0%, 15%);
  }
  :global(body.dark) {
    --back: hsl(0, 0%, 15%);
    --back-hover: hsl(0, 0%, 17%);
    --back-active: hsl(0, 0%, 20%);
    --back-two: hsl(0, 0%, 20%);
    --back-two-hover: hsl(0, 0%, 27%);
    --back-two-active: hsl(0, 0%, 34%);
    --back-highlight: hsl(195, 20%, 30%);

    --text: hsl(0, 0%, 75%);
    --text-strong: hsl(0, 0%, 85%);
  }
</style>
