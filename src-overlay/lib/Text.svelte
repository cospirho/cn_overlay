<script lang="ts">
    type ToneMarksMapping = {
      [key: string]: string;
    };
    import { invoke } from "@tauri-apps/api/tauri"
    import { LogicalSize, WebviewWindow } from '@tauri-apps/api/window'
    //todo https://tauri.app/v1/api/js/window/ communicate with other window
    // we'll need to do it eventually even if we decide to display the text in the same window
    let text = '找到了!白露大人!_在这!';
    let textArr = text.split('');
    console.log(textArr);
    let box = [[360, 196], [772, 196], [772, 248], [360, 248]];
    let height = box[2][1] - box[0][1];
    let width = box[1][0] - box[0][0];
    let numchars = text.length;
    let charwidth = width / numchars;
  
    function hoverChar(ix: number): void {
      console.log(ix);
      let char = document.getElementById("char" + ix);
      console.log(char);
      char?.classList.add("highlight");
    }
  
    function unhoverChar(ix: number): void {
      console.log(ix);
      let char = document.getElementById("char" + ix);
      char?.classList.remove("highlight");
    }
    async function get_result(): Promise<void> {
      let result = await invoke("lookup_sentence", {sentence: text});
      console.log(result);
    }
    get_result();
  </script>
  
  <div>
    {#each textArr as char, ix}
      <!--<span id="char{ix}" class="text" style="left: {ix * charwidth}px">{char}</span>-->
      <span id="char{ix}" class="text" on:mouseenter={() => hoverChar(ix)} on:mouseleave={() => unhoverChar(ix)} role="img">{char}</span>
    {/each}
  </div>
<style>
  .text{
    /* font size is set to charwidth */
    font-size: 31px;
  }
    /* svelte compiler removes unused css, doesn't remove global rules */
  :global(.highlight){
    color: red;
  }
</style>