<script lang="ts">
    type Definition = [number, string, string, string, number]
    import { invoke } from "@tauri-apps/api/tauri"
    import { LogicalSize, WebviewWindow } from '@tauri-apps/api/window'
    //todo https://tauri.app/v1/api/js/window/ communicate with other window
    // we'll need to do it eventually even if we decide to display the text in the same window
    export let text: string = '为什么每次都能';
    export let text_id: number = 1;
    let textArr: string[];
    // these will be fetched from the backend
    export let box: [number[], number[], number[], number[]] = [[360, 196], [772, 196], [772, 248], [360, 248]];

    let height: number;
    let width: number;
    let padding: number;
    let numchars: number;
    let charWidth: number;

    let definitions: Definition[] = [];
    let pinyin:string = '';
    let character:string = '';
    let definition:string = '';
    let definition_data:Definition;
  
    function hoverChar(ix: number): void {
      // todo optimize 
      for(var i = 0; i<definitions.length; i++){
        if(definitions[i][0] >= ix){
          if(definitions[i][0] == ix){
            definition_data = definitions[i]
          }else{
            definition_data = definitions[i-1];
          }
          pinyin = definition_data[1];
          definition = definition_data[2];
          character = definition_data[3];
          let startIndex = definition_data[0];
          let definitionLen = definition_data[4];
          for(var j = startIndex; j<startIndex+definitionLen; j++){
            let char = document.getElementById(text_id + "_" + j);
            char?.classList.add("highlight");
          }
          break;
        }
      }
    }
  
    function unhoverChar(ix: number): void {
      //remove highlight from all characters
      // could maybe just remove from ix-4 to ix + 4, with bounds checks
      for(var i = 0; i<=textArr.length; i++){
        let char = document.getElementById(text_id + "_" + i);
        char?.classList.remove("highlight");
      }
      character = "";
      pinyin = "";
      definition = "";
    }

    async function get_definitions(): Promise<void> {
      definitions = await invoke("lookup_sentence", {sentence: text});
      console.log(definitions);
    }

    async function handleMouseup(): Promise<void> {
      let selection = window.getSelection();
      let selectedText = selection?.toString();
      if (selectedText!.length < 1 || selectedText!.length > 10) {
        return;
      }
      character = selectedText!;
      let lookupResults:string[] = await invoke("lookup_word", {word: selectedText});
      pinyin = lookupResults[0];
      definition = lookupResults[1];
    }

    export function updateSentence(sentence: string, box: number[][]): void {
      text = sentence;
      textArr = text.split('');
      numchars = text.length;
      box = box;
      height = box[2][1] - box[0][1];
      width = box[1][0] - box[0][0];
      padding = height / 5;
      charWidth = width / numchars;
      get_definitions();
    }
  updateSentence(text, box);
  </script>
  
  <div id="definition" class="resultCard" role="img">
    <!--<span class="character">Character: {character}</span>-->
    <span class="pinyin">{pinyin}</span>
    <span class="definition">{definition}</span>
  </div>
  <div class="sentence" style="top: {box[0][1] + padding}px; left: {box[0][0]}px;" on:mouseup={handleMouseup} role="presentation">
    {#each textArr as char, ix}
      <span id="{text_id}_{ix}" class="text" style="font-size:{charWidth}px" on:mouseenter={() => hoverChar(ix)} on:mouseleave={() => unhoverChar(ix)} role="img">{char}</span>
    {/each}
  </div>
<style>
  .definition{
    display:block;
    background-color: black;
  }
  .pinyin{
    display:block;
    background-color: black;
  }
  .resultCard{
    position:inline-block;
    justify-content: center;
    align-items: center;
  }
  .sentence{
    display:block;
    position: absolute;
  }
  .text{
    background-color: black;
  }
    /* svelte compiler removes unused css, doesn't remove global rules */
  :global(.highlight){
    color: red;
  }
</style>