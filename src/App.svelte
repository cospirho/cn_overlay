<script lang="ts">
  import Greet from './lib/Greet.svelte'
  // make window not transparent
  import { appWindow } from "@tauri-apps/api/window";
  appWindow.setDecorations(true);
</script>

<main class="container">
  <p>
    
  </p>

  <div class="row">
    <Greet />
  </div>


</main>

<style>

</style>