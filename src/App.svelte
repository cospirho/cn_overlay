<script lang="ts">
  import SelectOverlay from './lib/SelectOverlay.svelte'
  // make window not transparent
  import { appWindow } from "@tauri-apps/api/window";
  appWindow.setDecorations(true);
</script>

<main class="container">
  <p>
    
  </p>

  <div class="row">
    <SelectOverlay />
  </div>


</main>

<style>

</style>