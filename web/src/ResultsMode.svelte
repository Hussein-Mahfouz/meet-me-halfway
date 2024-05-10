<script lang="ts">
  import SplitComponent from "./SplitComponent.svelte";
  import { mode } from "./stores";
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Results mode</h2>
    <div>
      <button on:click={() => ($mode = "title")}>Change study area</button>
      <button on:click={() => ($mode = "input")}>Change input</button>
    </div>
  </div>
  <div slot="map"></div>
</SplitComponent>
