<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { LogicalSize, WebviewWindow } from '@tauri-apps/api/window'
  let name = "";
  let greetMsg = ""

  async function createOverlay(){
    const webview = new WebviewWindow('overlay', {
      url: 'overlay.html',
      transparent: true,
      height: 700,
      width: 700,
      resizable: true,
      alwaysOnTop: true,
      decorations: false,
      title: "Select Overlay Area",
    });
  }
</script>

<div>
  <button type="button" on:click={createOverlay}>Select Overlay Area</button>
  <p>{greetMsg}</p>
</div>