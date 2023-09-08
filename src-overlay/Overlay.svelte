<script lang="ts">
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  type State = "set_overlay" | "run_overlay"
  let state: State = "set_overlay";

  function setOverlay(): void {
    appWindow.setDecorations(true);
    state = "set_overlay";  
  }
  async function getPosition(){
    const position = await appWindow.innerPosition();
    return position;
  }

  function confirmOverlay(): void {
    appWindow.setDecorations(false);
    state = "run_overlay";
    getPosition().then((position) => {
      console.log(position);
    });
  }

  setOverlay();

</script>

<main class="container">
  {#if state === "set_overlay"}
    <h2><span id="title">Move this window over the desired area</span></h2>
    <div class="row">
      <button type="button" class="confirm-btn" on:click={confirmOverlay}>Confirm</button>
    </div>
  {:else if state === "run_overlay"}
    <h2><span id="title">Overlay is running</span></h2>
    <button type="button" class="confirm-btn" on:click={setOverlay}>Close</button>
  {/if}
</main>

<style>
  #title {
    text-shadow: 2px 2px 50px rgba(238, 238, 244);
    padding: 10px;
    border-radius: 20px;
    background-color: rgb(206, 206, 206);
    color: black;
  }

  .confirm-btn{
    background-color: #0f0f0f;
  }

  @media only screen and (max-width: 526px) { 
  #title { 
    display: none; 
  } 
}
</style>