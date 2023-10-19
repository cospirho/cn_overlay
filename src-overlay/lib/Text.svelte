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
  
    function addTones(withNumbers: string): string {
      const toneMarks: ToneMarksMapping = {
        '1': '&#x0101;', '2': '&aacute;', '3': '&#x01ce;', '4': '&#x00e0;', '5': '',
        'a1': 'ā', 'a2': 'á', 'a3': 'ǎ', 'a4': 'à', 'a5': 'a',
        'e1': 'ē', 'e2': 'é', 'e3': 'ě', 'e4': 'è', 'e5': 'e',
        'i1': 'ī', 'i2': 'í', 'i3': 'ǐ', 'i4': 'ì', 'i5': 'i',
        'o1': 'ō', 'o2': 'ó', 'o3': 'ǒ', 'o4': 'ò', 'o5': 'o',
        'u1': 'ū', 'u2': 'ú', 'u3': 'ǔ', 'u4': 'ù', 'u5': 'u',
        'u:1': 'ǖ', 'u:2': 'ǘ', 'u:3': 'ǚ', 'u:4': 'ǜ', 'u:5': 'ü',
        'u:': 'ü'
      };

      // Regular expression to match Pinyin with numbers
      const pinyinWithNumbersRegex = /([a-z\u00E0-\u00FC]+)(\d)/g;

      // Replace Pinyin with numbers with tone marks
      const result = withNumbers.replace(pinyinWithNumbersRegex, (_, pinyin:string, tone:string) => {
        const toneMark = toneMarks[`${pinyin}${tone}`] || '';
        return toneMark;
      });

      return result;
    }
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