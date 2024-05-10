<script lang="ts">
  import SplitComponent from "./SplitComponent.svelte";
  import { mode } from "./stores";

  export let data: any;
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Results mode</h2>
    <div>
      <button on:click={() => ($mode = { kind: "title" })}
        >Change study area</button
      >
      <button on:click={() => ($mode = { kind: "input" })}>Change input</button>
    </div>

    <pre>{JSON.stringify(data)}</pre>
  </div>
  <div slot="map"></div>
</SplitComponent>
