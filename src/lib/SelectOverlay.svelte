<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { LogicalSize, WebviewWindow, appWindow } from '@tauri-apps/api/window'

  async function createOverlay(){
    const webview = new WebviewWindow('overlay', {
      url: 'index.html',
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
  <button type="button" on:click={createOverlay}>Select Overlay Area...</button>
</div>