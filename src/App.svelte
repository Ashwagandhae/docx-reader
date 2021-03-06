<script lang="ts">
  import { invoke } from '@tauri-apps/api';
  import { fade } from 'svelte/transition';
  import { onMount, tick } from 'svelte';
  import Doc from './Doc.svelte';
  import Outline from './Outline.svelte';
  import Topbar from './Topbar.svelte';
  import SearchResults from './SearchResults.svelte';
  import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
  import { listen } from '@tauri-apps/api/event';
  import { setContext } from 'svelte';
  import { writable } from 'svelte/store';
  import { register } from './shortcut';
  import type { OutlineParaType, ParaType, LoaderState } from './types';
  import type { Writable } from 'svelte/store';

  onMount(async () => {
    let ready: { file_path: string } = await invoke('window_ready', {});
    if (ready != null) {
      if (ready.file_path != null) loadFile(ready.file_path);
    }
  });
  // listen for changes in system settings
  let colorThemeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  let updateColorTheme = function () {
    document.body.classList.toggle('dark', colorThemeMediaQuery.matches);
  };
  updateColorTheme();
  colorThemeMediaQuery.addEventListener('change', updateColorTheme);

  async function chooseFile() {
    let path: string = await invoke('open_dialog');
    if (!path) return;
    loadFile(path);
  }
  let droppingFile = false;
  listen('tauri://file-drop-hover', () => {
    droppingFile = true;
  });
  listen('tauri://file-drop-cancelled', () => {
    droppingFile = false;
  });
  listen('tauri://file-drop', (event: { payload: string[] }) => {
    droppingFile = false;
    if (event.payload.length > 0) {
      loadFile(event.payload[0]);
    }
    if (event.payload.length > 1) {
      invoke('new_window', {
        creates: event.payload
          .slice(1, event.payload.length)
          .map(function (path) {
            return {
              file_path: path,
            };
          }),
      });
    }
  });
  listen('load_file', (event: { payload: { message: string } }) => {
    loadFile(event.payload.message as string);
  });
  listen('tauri://focus', (event: { payload: boolean }) => {
    event.payload && invoke('window_focus', {});
  });
  let fileInfo = writable({
    open: false,
    path: '',
    name: '',
  });
  setContext('fileInfo', fileInfo);
  async function loadFile(path: string) {
    let extension = path.split('.').pop();
    if (extension != 'docx') return;
    await closeFile();
    let fileResult = await invoke('load_file', { path });
    if (!fileResult) return;
    $fileInfo = {
      open: true,
      path,
      name: path.split('/').pop(),
    };
    // set title of window
    appWindow.setTitle($fileInfo.name);
    doc.getLoader().teleport(0, true);
    outline?.getLoader()?.teleport(0, true);
    searchResults?.getLoader()?.teleport(0, true);
  }
  async function closeFile() {
    await Promise.all([invoke('unload_file'), invoke('clear_search')]);
    $fileInfo = {
      open: false,
      path: '',
      name: '',
    };
    await tick();
  }
  let query = writable({
    text: '',
    matchCase: false,
    onlyOutline: false,
  });
  setContext('query', query);
  let selectedQuery = writable({
    paraIndex: null,
    charIndex: null,
  });
  setContext('selectedQuery', selectedQuery);

  function getDocLoader() {
    return doc.getLoader();
  }
  setContext('getDocLoader', getDocLoader);

  let doc: Doc;
  let outline: Outline;
  let searchResults: SearchResults;

  let showSearchResults = true;
  let showOutline = true;

  let resizeTimer: NodeJS.Timeout;
  let isResizing = writable(false);
  setContext('isResizing', isResizing);
  let isFullscreen = writable(false);
  setContext('isFullscreen', isFullscreen);
  function resizeHandler() {
    clearTimeout(resizeTimer);
    $isResizing = true;
    resizeTimer = setTimeout(function () {
      $isResizing = false;
      invoke('get_window_fullscreen_state', {}).then((result: boolean) => {
        $isFullscreen = result;
      });
    }, 600);
  }
  function prevResult() {
    searchResults?.prevResult();
  }
  setContext('prevResult', prevResult);
  function nextResult() {
    searchResults?.nextResult();
  }
  setContext('nextResult', nextResult);
  register('CommandOrControl+N', () => {
    invoke('new_window', { creates: [null] });
  });

  async function alignOutlineFocus() {
    let docFocus: ParaType = doc.getLoader().getFocus();
    let para: OutlineParaType = await invoke('get_nearest_outline_para', {
      paraIndex: docFocus.index,
    });
    console.log('outline aligning to: ', para);
    if (para) {
      outline.getLoader().teleport(para.index);
    }
  }

  let zoom = writable(1);
  setContext('zoom', zoom);

  let states = {
    doc: {
      loader: null,
    },
    outline: {
      loader: null,
    },
    searchResults: {
      loader: null,
      matchCase: false,
      onlyOutline: false,
    },
  };
</script>

<svelte:window on:resize={resizeHandler} />
<main>
  {#if droppingFile}
    <div class="screen" transition:fade={{ duration: 200 }}>
      <div class="message">drop file here</div>
    </div>
  {/if}

  <div class="topbar">
    <Topbar
      bind:showOutline
      bind:showSearchResults
      matchCase={states.searchResults.matchCase}
      onlyOutline={states.searchResults.onlyOutline}
      {chooseFile}
      {alignOutlineFocus}
    />
  </div>
  <div class="doc">
    <Doc
      bind:this={doc}
      {showOutline}
      {showSearchResults}
      bind:state={states.doc}
    />
  </div>
  <aside class="outline">
    <Outline bind:this={outline} {showOutline} bind:state={states.outline} />
  </aside>
  <aside class="search-results">
    <SearchResults
      bind:this={searchResults}
      {showSearchResults}
      bind:state={states.searchResults}
    />
  </aside>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: var(--font-family);
    color: var(--text);
    background-color: var(--back);
    width: 100vw;
    height: 100vh;
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
  .topbar {
    width: 100vw;
    z-index: 10000;
    position: absolute;
    height: var(--topbar-height);
    cursor: default;
  }
  .doc {
    position: relative;
    height: 100vh;
    width: 100vw;
    grid-area: doc;
    box-sizing: border-box;
    padding-top: var(--topbar-height);
  }
  aside {
    width: var(--sidebar-width);
    height: auto;
    position: absolute;
    top: 0;
    z-index: 100;
  }
  .outline {
    left: 0;
  }
  .search-results {
    right: 0;
    pointer-events: none;
  }
  :global(body) {
    --padding: 10px;
    --padding-small: 5px;
    --topbar-height: calc(2em + var(--padding));
    --gap: 0px;
    --bold: 600;
    --border-radius: 10px;
    --sidebar-width: max(150px, 15vw);
    --font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
      Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji',
      'Segoe UI Symbol';
    --transition-speed: 300ms;
  }
  :global(body.dark) {
    --back: hsl(0, 0%, 15%);
    --back-hover: hsl(0, 0%, 17%);
    --back-active: hsl(0, 0%, 20%);

    --back-two: hsl(0, 0%, 20%);
    --back-two-hover: hsl(0, 0%, 27%);
    --back-two-active: hsl(0, 0%, 34%);
    --back-highlight: hsl(195, 20%, 30%);

    --back-mark-c: 52, 76%, 30%;
    --back-mark: hsl(var(--back-mark-c));
    --back-mark-selected-c: 30, 80%, 40%;
    --back-mark-selected: hsl(var(--back-mark-selected-c));

    --text: hsl(0, 0%, 75%);
    --text-weak: hsl(0, 0%, 50%);
    --text-strong: hsl(0, 0%, 85%);

    --shadow: rgba(0, 0, 0, 0.4) 0px 7px 29px 0px;
    --shadow-small: rgba(0, 0, 0, 0.2) 0px 7px 10px 0px;
  }
  :global(body) {
    --back: hsl(0, 0%, 100%);
    --back-hover: hsl(0, 0%, 97%);
    --back-active: hsl(0, 0%, 94%);

    --back-two: hsl(0, 0%, 95%);
    --back-two-hover: hsl(0, 0%, 89%);
    --back-two-active: hsl(0, 0%, 80%);
    --back-highlight: hsl(195, 100%, 80%);

    --back-mark-c: 60, 100%, 69%;
    --back-mark: hsl(var(--back-mark-c));
    --back-mark-selected-c: 40, 100%, 59%;
    --back-mark-selected: hsl(var(--back-mark-selected-c));

    --text: hsl(0, 0%, 30%);
    --text-weak: hsl(0, 0%, 50%);
    --text-strong: hsl(0, 0%, 10%);

    --shadow: rgba(0, 0, 0, 0.2) 0px 7px 29px 0px;
    --shadow-small: rgba(0, 0, 0, 0.05) 0px 7px 10px 0px;
  }
</style>
