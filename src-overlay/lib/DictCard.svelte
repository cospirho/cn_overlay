<script lang="ts">
    type ToneMarksMapping = {
      [key: string]: string;
    };
    import { invoke } from "@tauri-apps/api/tauri"
    import { LogicalSize, WebviewWindow } from '@tauri-apps/api/window'
    let hanzi = '一眼看穿';
    let pinyin = "yī yǎn kàn chuān"
    let definition = "to see through something at first glance (idiom)"
  
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