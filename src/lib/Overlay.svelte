<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  import Text from "./Text.svelte";
  type State = "set_overlay" | "select_process" | "run_overlay"
  type Box = [number[], number[], number[], number[]];
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
    appWindow.setResizable(true);
    document.body.style.backgroundColor = "rgba(238, 238, 244, 0.4)";
    state = "set_overlay";  
  }

  async function getPosition(){
    const position = await appWindow.innerPosition();
    return position;
  }

  function confirmOverlay(): void {
    appWindow.setDecorations(false);
    appWindow.setResizable(false);
    state = "run_overlay";
    document.body.style.backgroundColor = "rgba(238, 238, 244, 0.03)";
    getPosition().then((position) => {
      overlay_xy = [position.x - windowLeft, position.y - windowTop];
    });
    appWindow.innerSize().then((size) => {
      let width = size.width;
      let height = size.height;
      overlay_wh = [width, height];
    });
  }

  function selectProcess(): void {
    appWindow.setDecorations(true);
    appWindow.setResizable(true);
    state = "select_process";
    document.body.style.backgroundColor = "#2f2f2f";
    windowInterval = setInterval(getMouseWindow, 300);
  }

  async function takeScreenshot(){
    let result: any = await invoke("screenshot", {windowId: windowId, cropWh: overlay_wh, cropXy: overlay_xy});
    result = JSON.parse(result);
    document.getElementById("overlayText")!.innerHTML = "";
    for(var i = 0; i<result["texts"].length; i++){
      let text = result["texts"][i];
      let box = result["boxes"][i];
      addText(text, box, i);
    }
  }

  async function getMouseWindow(): Promise<void> {
    let result: [number, String, number, number, number, number] = await invoke("get_window");
    [windowId, windowName, windowLeft, windowTop, windowRight, windowBottom] = result;
    if(windowName === ""){
      windowName = "No title - " + windowId.toString();
    }
    if(!document.hasFocus()){
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

  function addText(text:string, box:Box, text_id:number): void {
    const textComponent = new Text({
      target: document.getElementById("overlayText")!,
      props: {
        text: text,
        box: box,
        text_id: text_id
      }
    });
  }
  
  function clearText(){
    document.getElementById("overlayText")!.innerHTML = "";
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
    <div class="left-overlay-buttons">
      <button type="button" class="confirm-btn overlay-btn" on:click={setOverlay}>Close</button>
    </div>
    <div class="overlay-buttons">
      <button type="button" class="confirm-btn overlay-btn" on:click={clearText}>Clear</button>
      <button type="button" class="confirm-btn overlay-btn" on:click={takeScreenshot}>Take Screenshot</button>
    </div>
    <div id="overlayText">
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
  .overlay-btn{
    opacity:0.1;
  }
  .overlay-btn:hover{
    opacity:1;
  }
  .overlay-buttons{
    position: absolute;
    top: 0;
    right: 0;
  }
  .left-overlay-buttons{
    position: absolute;
    top: 0;
    left: 0;
  }
  @media only screen and (max-width: 526px) { 
  #title { 
    display: none; 
  } 
}
</style>