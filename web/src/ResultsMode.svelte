<script lang="ts">
  import {
    GeoJSON,
    CircleLayer,
    Marker,
    hoverStateFilter,
  } from "svelte-maplibre";
  import {
    SequentialLegend,
    makeColorRamp,
    Popup,
    notNull,
  } from "svelte-utils";
  import SplitComponent from "./SplitComponent.svelte";
  import { mode, model, type Person, type POI, averageTime } from "./stores";
  import type { FeatureCollection } from "geojson";

  export let people: Person[];

  let pois = calculate(people);

  function calculate(people: Person[]): POI[] {
    try {
      let pois: POI[] = JSON.parse($model!.findPOIs({ people }));
      // Sort by the average time, just to make the list less intense
      pois.sort((a, b) => averageTime(a) - averageTime(b));
      return pois;
    } catch (err) {
      window.alert(`Bug: ${err}`);
      $mode = { kind: "input", people };
      return [];
    }
  }

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
            kind: poi.kind,
            name: poi.name,
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
      <button on:click={() => ($mode = { kind: "input", people })}
        >Change input</button
      >
    </div>

    <SequentialLegend {colorScale} limits={limitsMinutes} />

    <ul>
      {#each pois as poi}
        <li>
          <a href={poi.osm_url} target="_blank"
            >{poi.name || "Unnamed"} ({poi.kind})</a
          >
          <span>{JSON.stringify(poi.times_per_person)}</span>
        </li>
      {/each}
    </ul>
  </div>

  <div slot="map">
    <GeoJSON data={toGj(pois)} generateId>
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
          <h5>{props.name || "Unnamed"} ({props.kind})</h5>
          {#each JSON.parse(props.times_per_person) as [name, seconds]}
            <p>{name}: {Math.round(seconds / 60)} minutes</p>
          {/each}
        </Popup>
      </CircleLayer>

      {#each people as person, idx}
        <Marker lngLat={person.home}>
          <span class="dot">
            {idx + 1}
          </span>
        </Marker>
      {/each}
    </GeoJSON>
  </div>
</SplitComponent>

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: inline-block;

    background-color: red;
    color: white;
    border: 3px solid white;
    text-align: center;
    /* TODO Weird way to vertically align */
    line-height: 250%;
  }
</style>
