<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { LogicalSize, WebviewWindow } from '@tauri-apps/api/window'
  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  async function createOverlay(){
    const webview = new WebviewWindow('theUniqueLabel', {
      url: 'overlay.html',
      transparent: true,
      height: 700,
      width: 700,
      resizable: true,
      decorations: false,
      alwaysOnTop: true,
    });
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <button type="button" on:click={createOverlay}>Select Overlay Area</button>
  <p>{greetMsg}</p>
</div>