<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  type State = "set_overlay" | "select_process" | "run_overlay"
  let state: State = "select_process";
  let windowName:String = "No process selected";
  let windowId: number = 0;
  let windowInterval: number;
  let overlay_wh: [number, number] = [0, 0];
  let overlay_xy: [number, number] = [0, 0];

  let windowLeft: number;
  let windowTop: number;
  let windowRight: number;
  let windowBottom: number;
    

  function setOverlay(): void {
    appWindow.setDecorations(true);
    document.body.style.backgroundColor = "rgba(238, 238, 244, 0.4)";
    state = "set_overlay";  
  }

  async function getPosition(){
    const position = await appWindow.innerPosition();
    return position;
  }

  function confirmOverlay(): void {
    appWindow.setDecorations(false);
    state = "run_overlay";
    document.body.style.backgroundColor = "rgba(238, 238, 244, 0)";
    getPosition().then((position) => {
      console.log(position);
      overlay_xy = [position.x - windowLeft, position.y - windowTop];
      console.log(overlay_xy);
    });
    console.log(overlay_xy);
    appWindow.innerSize().then((size) => {
      console.log(size);
      let width = size.width;
      let height = size.height;
      overlay_wh = [width, height];
    });
  }

  function selectProcess(): void {
    appWindow.setDecorations(true);
    state = "select_process";
    document.body.style.backgroundColor = "#2f2f2f";
    windowInterval = setInterval(getMouseWindow, 300);
  }

  async function takeScreenshot(){
    let result = await invoke("screenshot", {windowId: windowId, cropWh: overlay_wh, cropXy: overlay_xy});
    console.log("took screenshitjkjA" + result);
  }

  async function getMouseWindow(): Promise<void> {
    let result: [number, String, number, number, number, number] = await invoke("get_window");
    [windowId, windowName, windowLeft, windowTop, windowRight, windowBottom] = result;
    console.log(result);
    if(windowName === ""){
      windowName = "No title - " + windowId.toString();
    }
    if(!document.hasFocus()){
      console.log("lost focus");
      document.getElementById("lostfocus")!.classList.remove("invisible");
    }else{
      document.getElementById("lostfocus")!.classList.add("invisible");
    }
  }

  async function onKeyDown(evt: KeyboardEvent){
    if(state === "select_process"){
      if(evt.key === " "){
        await getMouseWindow();
        clearInterval(windowInterval);
        setOverlay();
      }
    }
  }
  selectProcess();
</script>

<main class="container">
  {#if state === "select_process"}
    <h1 id="lostfocus" class="invisible">Lost focus! Please click back on this window</h1>
    <h3>Will overlay this window:</h3>
    <h3>{windowName}</h3>
    <h4>Mouse over the desired process and press space to select it</h4>
  {:else if state === "set_overlay"}
    <h2><span id="title">Move this window over the desired area</span></h2>
    <div class="row">
      <button type="button" class="confirm-btn" on:click={selectProcess}>Cancel</button>
      <button type="button" class="confirm-btn" on:click={confirmOverlay}>Confirm</button>
    </div>
  {:else if state === "run_overlay"}
    <h2><span id="title">Overlay is running</span></h2>
    <div class = "row">
      <button type="button" class="confirm-btn" on:click={setOverlay}>Close</button>
      <button type="button" class="confirm-btn" on:click={takeScreenshot}>Take Screenshot</button>
    </div>
  {/if}
</main>

<svelte:window on:keydown|preventDefault={onKeyDown} />
<style>
  #title {
    text-shadow: 2px 2px 50px rgba(238, 238, 244);
    padding: 10px;
    border-radius: 20px;
    background-color: rgb(206, 206, 206);
    color: black;
  }

  #lostfocus{
    color: red;
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