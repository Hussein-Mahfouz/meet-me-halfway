<script lang="ts">
  import { Marker } from "svelte-maplibre";
  import type { MapMouseEvent } from "maplibre-gl";
  import { onMount, onDestroy } from "svelte";
  import SplitComponent from "./SplitComponent.svelte";
  import { mode, type Person, colours } from "./stores";
  import { map } from "./stores";

  export let people: Person[];

  // TODO Wait for loaded
  onMount(() => {
    $map?.on("click", onMapClick);
  });
  onDestroy(() => {
    $map?.off("click", onMapClick);
  });

  function onMapClick(e: MapMouseEvent) {
    people = [
      ...people,
      {
        name: "",
        home: e.lngLat.toArray(),
        maxTimeMinutes: 60,
      },
    ];
  }

  function deletePerson(idx: number) {
    people.splice(idx, 1);
    people = people;
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Input mode</h2>
    <div>
      <button on:click={() => ($mode = { kind: "title" })}
        >Change study area</button
      >
    </div>

    <p>Click the map to add people</p>
    <button
      disabled={people.length == 0}
      on:click={() => {
        $mode = { kind: "results", people };
      }}>Calculate</button
    >

    {#each people as person, idx}
      <h2>Person {idx + 1}</h2>
      <label>
        Name: <input type="text" bind:value={person.name} />
      </label>
      <label>
        Max travel time (minutes): {person.maxTimeMinutes}
        <input
          type="range"
          min="5"
          max="120"
          bind:value={person.maxTimeMinutes}
        />
      </label>
      <button on:click={() => deletePerson(idx)}>Delete</button>
    {/each}
  </div>

  <div slot="map">
    {#each people as person, idx}
      <Marker draggable bind:lngLat={person.home}>
        <span class="dot" style="background-color: {colours[idx % colours.length]}">
          {idx + 1}
        </span>
      </Marker>
    {/each}
  </div>
</SplitComponent>

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;

    color: white;
    border: 3px solid white;

    display: flex;
    justify-content: center;
    align-items: center;
  }
  .dot:hover {
    border: 6px solid white;
    cursor: pointer;
  }
</style>
