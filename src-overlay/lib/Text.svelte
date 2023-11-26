<script lang="ts">
    type Definition = [number, string, string, string, number]
    import { invoke } from "@tauri-apps/api/tauri"
    import { LogicalSize, WebviewWindow } from '@tauri-apps/api/window'
    //todo https://tauri.app/v1/api/js/window/ communicate with other window
    // we'll need to do it eventually even if we decide to display the text in the same window
    let text = '找到了!白露大人!_在这!';
    let textArr = text.split('');
    // these will be fetched from the backend
    let box = [[360, 196], [772, 196], [772, 248], [360, 248]];
    let height = box[2][1] - box[0][1];
    let width = box[1][0] - box[0][0];
    let numchars = text.length;
    //TODO set the font size to be the same as the width of the character
    let charwidth = width / numchars;

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
            let char = document.getElementById("char" + j);
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
        let char = document.getElementById("char" + i);
        char?.classList.remove("highlight");
      }
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

    get_definitions();
  </script>
  
  <div id="definition" class="resultCard" role="img">
    <span class="character">Character: {character}</span>
    <span class="pinyin">Pinyin: {pinyin}</span>
    <span class="definition">Definition: {definition}</span>
  </div>
  <div class="sentence" on:mouseup={handleMouseup} role="presentation">
    {#each textArr as char, ix}
      <!--<span id="char{ix}" class="text" style="left: {ix * charwidth}px">{char}</span>-->
      <span id="char{ix}" class="text" on:mouseenter={() => hoverChar(ix)} on:mouseleave={() => unhoverChar(ix)} role="img">{char}</span>
    {/each}
  </div>
<style>
  .definition{
    display:block;
  }
  .pinyin{
    display:block;
  }
  .resultCard{
    position:inline-block;
    justify-content: center;
    align-items: center;
  }
  .sentence{
    display:block;
  }
  .text{
    /* font size is set to charwidth */
    font-size: 31px;
  }
    /* svelte compiler removes unused css, doesn't remove global rules */
  :global(.highlight){
    color: red;
  }
</style>