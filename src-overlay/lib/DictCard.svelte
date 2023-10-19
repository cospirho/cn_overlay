<script lang="ts">
    type ToneMarksMapping = {
      [key: string]: string;
    };
    import { invoke } from "@tauri-apps/api/tauri"
    import { LogicalSize, WebviewWindow } from '@tauri-apps/api/window'
    let hanzi = '一眼看穿';
    let pinyin = "yī yǎn kàn chuān"
    let definition = "to see through something at first glance (idiom)"
  
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
   
  </script>
  <div class="dictCard">
  <div class="hanzi">{hanzi}</div>
  <div class="pinyin">{pinyin}</div>
  <div class="definition">{definition}</div>
  <button>Add</button>
  </div> 
  
<style>
  .text{

  }
</style>