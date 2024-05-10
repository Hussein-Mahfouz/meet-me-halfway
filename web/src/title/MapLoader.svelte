<script lang="ts">
  import { MapModel } from "backend";
  import { Loading, OverpassSelector } from "svelte-utils";
  import { map, model } from "../stores";

  let loading = "";

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loadModel(await fileInput.files![0].arrayBuffer());
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    loading = "";
  }

  function loadModel(buffer: ArrayBuffer) {
    loading = "Building map model from OSM input";
    console.time("load");
    $model = new MapModel(new Uint8Array(buffer));
    console.timeEnd("load");
  }

  function gotXml(e: CustomEvent<string>) {
    try {
      // TODO Can we avoid turning into bytes?
      loadModel(new TextEncoder().encode(e.detail));
    } catch (err) {
      window.alert(`Couldn't import from Overpass: ${err}`);
    }
    loading = "";
  }
</script>

<Loading {loading} />

<div>
  <label>
    Load an osm.xml or a .pbf file:
    <input bind:this={fileInput} on:change={loadFile} type="file" />
  </label>
</div>

<i>or...</i>

<OverpassSelector
  map={$map}
  on:gotXml={gotXml}
  on:loading={(e) => (loading = e.detail)}
  on:error={(e) => window.alert(e.detail)}
/>
