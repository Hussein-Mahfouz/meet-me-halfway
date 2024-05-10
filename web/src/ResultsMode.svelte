<script lang="ts">
  import { GeoJSON, CircleLayer, hoverStateFilter } from "svelte-maplibre";
  import {
    SequentialLegend,
    makeColorRamp,
    PropertiesTable,
    Popup,
    notNull,
  } from "svelte-utils";
  import SplitComponent from "./SplitComponent.svelte";
  import { mode, type POI, averageTime } from "./stores";
  import type { FeatureCollection } from "geojson";

  export let data: POI[];

  // TODO Should be based on the input
  let limitsMinutes = [0, 12, 24, 36, 48, 60];
  let limitsSeconds = limitsMinutes.map((x) => x * 60);
  let colorScale = ["#CDE594", "#80C6A3", "#1F9EB7", "#186290", "#080C54"];

  function toGj(data: POI[]): FeatureCollection {
    return {
      type: "FeatureCollection",
      features: data.map((poi) => {
        return {
          type: "Feature",
          properties: {
            osm_url: poi.osm_url,
            kind: poi.osm_url,
            name: poi.osm_url,
            times_per_person: poi.times_per_person,
            average: averageTime(poi),
          },
          geometry: {
            type: "Point",
            coordinates: [poi.point.x, poi.point.y],
          },
        };
      }),
    };
  }
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

    <SequentialLegend {colorScale} limits={limitsMinutes} />

    <ul>
      {#each data as poi}
        <li>
          <a href={poi.osm_url} target="_blank"
            >{poi.name || "Unnamed"} ({poi.kind}</a
          >
          <span>{JSON.stringify(poi.times_per_person)}</span>
        </li>
      {/each}
    </ul>
  </div>

  <div slot="map">
    <GeoJSON data={toGj(data)} generateId>
      <CircleLayer
        paint={{
          "circle-radius": 5,
          "circle-opacity": 0,
          "circle-stroke-width": hoverStateFilter(2, 4),
          "circle-stroke-color": makeColorRamp(
            ["get", "average"],
            limitsSeconds,
            colorScale,
          ),
        }}
        manageHoverState
        on:click={(e) =>
          window.open(
            notNull(e.detail.features[0].properties).osm_url,
            "_blank",
          )}
        hoverCursor="pointer"
      >
        <Popup openOn="hover" let:props>
          <PropertiesTable properties={props} />
        </Popup>
      </CircleLayer>
    </GeoJSON>
  </div>
</SplitComponent>
