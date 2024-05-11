<script lang="ts">
  import { type POI } from "./stores";
  import type { Feature, Point, FeatureCollection } from "geojson";
  export let poi: POI;
  export let hoveredAmenity: Feature<Point> | null;
  export let gjData: FeatureCollection<Point>;

  function setHoveredAmenity(poi: POI) {
    let feat = gjData.features.find((f) => f.properties!.osm_url === poi.osm_url);
    if (feat) {
      console.log(feat);
      hoveredAmenity = feat;
    }
    else {
      console.log("Couldn't find feature for", poi);
    }
  }
</script>

<div
    class="poi"
    on:mouseover={() => setHoveredAmenity(poi)}
    on:mouseout={() => {hoveredAmenity = null;}}
>
    <p>{poi.name || "Unnamed"} ({poi.kind})</p>

    <div class="costs">
        {#each poi.times_per_person as [name, cost]}
            <div class="person_times">{name} {cost}</div>
        {/each}
    </div>

    <p><a class="osm_link" href={poi.osm_url} target="_blank">View on OpenStreetMap</a></p>
</div>

<style>
  .poi {
    padding: 10px;
    border: 2px solid black;
    border-radius: 5px;
  }

  .poi:hover {
    background-color: #eee;
  }

  .costs {
    display: flex;
    gap: 5px;
  }

  a.osm_link {
    color: blue;
    font-size: 80%;
  }

  p {
    margin: 0;
  }
</style>
