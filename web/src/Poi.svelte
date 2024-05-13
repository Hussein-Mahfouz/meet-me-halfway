<script lang="ts">
  import { type POI, map } from "./stores";
  import type { Feature, Point, FeatureCollection } from "geojson";

  export let poi: POI;
  export let hoveredAmenity: Feature<Point> | null;
  export let gjData: FeatureCollection<Point>;

  function setHoveredAmenity(poi: POI) {
    let feat = gjData.features.find((f) => f.properties!.osm_url === poi.osm_url);
    if (feat) {
      hoveredAmenity = feat;
    }
    else {
      console.log("Couldn't find feature for", poi);
    }
  }

  function centreMapOnAmenity(poi: POI) {
    if ($map) {
      $map.flyTo({center: [poi.point.x, poi.point.y]});
    }
  }
</script>

<div
    class="poi"
    on:mouseenter={() => setHoveredAmenity(poi)}
    on:mouseleave={() => {hoveredAmenity = null;}}
    on:click={() => centreMapOnAmenity(poi)}
    role="tooltip"
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
    cursor: pointer;
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
