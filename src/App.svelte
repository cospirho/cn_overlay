<script lang="ts">
  import SelectOverlay from './lib/SelectOverlay.svelte'
  import Overlay from "./lib/Overlay.svelte";
  // make window not transparent
  import { appWindow } from "@tauri-apps/api/window";
  appWindow.title().then((title) => {
    if(title == "Select Overlay Area"){
      document.body.innerHTML = "";
      //create Overlay component here
      const overlay = new Overlay({
        target: document.body,
      });
    }
  });
</script>

<main class="container">
  <SelectOverlay />
</main>

<style>

</style>