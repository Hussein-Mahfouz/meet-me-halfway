<script lang="ts">
  import {
    GeoJSON,
    CircleLayer,
    Marker,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import {
    SequentialLegend,
    makeColorRamp,
    Popup,
    notNull,
  } from "svelte-utils";
  import SplitComponent from "./SplitComponent.svelte";
  import Poi from "./Poi.svelte";
  import { mode, model, type Person, type POI, colours } from "./stores";
  import type { Feature, Point, FeatureCollection } from "geojson";

  export let people: Person[];

  let pois = calculate(people);

  let sortBy: "max" | "sum" = "sum";

  function sumList(list: number[]): number {
    let result = 0;
    for (let x of list) {
      result += x;
    }
    return result;
  }
  let sortFunction = {
    max: (poi: POI) =>
      Math.max(...poi.times_per_person.map(([name, cost]) => cost)),
    sum: (poi: POI) =>
      sumList(poi.times_per_person.map(([name, cost]) => cost)),
  };

  $: resortData(sortBy);

  function resortData(sortBy: "max" | "sum") {
    pois.sort((a, b) => sortFunction[sortBy](a) - sortFunction[sortBy](b));
    pois = pois;
  }

  function calculate(people: Person[]): POI[] {
    try {
      return JSON.parse($model!.findPOIs({ people }));
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

  function toGj(data: POI[]): FeatureCollection<Point> {
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
            score: sortFunction[sortBy](poi),
          },
          geometry: {
            type: "Point",
            coordinates: [poi.point.x, poi.point.y],
          },
        };
      }),
    };
  }

  let gjData = toGj(pois);
  let hoveredAmenity: Feature<Point> | null;
  let routeGj: FeatureCollection[] | null = null;
  $: if (hoveredAmenity) {
    try {
      routeGj = JSON.parse(
        $model!.routesTo({
          people,
          point: hoveredAmenity.geometry.coordinates,
        }),
      );
    } catch (err: any) {
      routeGj = null;
      console.error(`routesTo broke: ${err}`);
    }
  } else {
    routeGj = null;
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

    <select bind:value={sortBy}>
      <option value="sum">Sum of travel time of everyone</option>
      <option value="max">Maximum / worst-case travel time of anyone</option>
    </select>

    <div class="poi_container">
      {#each pois as poi}
            <Poi {poi} {gjData} bind:hoveredAmenity />
      {/each}
    </div>
  </div>

  <div slot="map">
    <GeoJSON data={gjData} generateId>
      <CircleLayer
        paint={{
          "circle-radius": 5,
          "circle-opacity": 0,
          "circle-stroke-width": hoverStateFilter(2, 4),
          "circle-stroke-color": makeColorRamp(
            ["get", "score"],
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
        bind:hovered={hoveredAmenity}
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
          <span class="dot" style="background-color: {colours[idx % colours.length]}">
            {idx + 1}
          </span>
        </Marker>
      {/each}

      {#if routeGj}
        {#each routeGj as route, idx}
          <GeoJSON data={route}>
            <LineLayer
              paint={{
                "line-width": 10,
                "line-color": colours[idx % colours.length],
              }}
            />
          </GeoJSON>
        {/each}
      {/if}
    </GeoJSON>
  </div>
</SplitComponent>

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;

    color: white;
    border: 3px solid white;
    text-align: center;

    display: flex;
    justify-content: center;
    align-items: center;
  }

  .poi_container {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
</style>
