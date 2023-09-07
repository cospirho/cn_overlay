<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  async function createWindow(){
    greetMsg = await invoke("open_docs");
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <button type="button" on:click={createWindow}>Greet</button>
  <p>{greetMsg}</p>
</div>