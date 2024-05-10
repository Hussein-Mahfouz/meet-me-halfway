<script lang="ts">
  import SplitComponent from "./SplitComponent.svelte";
  import { mode } from "./stores";
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Input mode</h2>
    <div>
      <button on:click={() => ($mode = "title")}>Change study area</button>
    </div>
  </div>
  <div slot="map"></div>
</SplitComponent>
