import { MapModel } from "backend";
import type { Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export let sidebarContents: Writable<HTMLDivElement | null> = writable(null);
export let mapContents: Writable<HTMLDivElement | null> = writable(null);

export interface Person {
  name: string;
  home: [number, number];
  maxTimeMinutes: number;
}

export interface POI {
  osm_url: string;
  point: {
    x: number;
    y: number;
  };
  kind: string;
  name: string | null;
  times_per_person: [string, number][];
}

export type Mode =
  | { kind: "title" }
  | { kind: "input"; people: Person[] }
  | { kind: "results"; people: Person[] };

export let mode: Writable<Mode> = writable({ kind: "title" });
export let model: Writable<MapModel | null> = writable(null);
export let map: Writable<Map | null> = writable(null);
export let showAbout: Writable<boolean> = writable(true);

export function averageTime(poi: POI): number {
  let sum = 0;
  for (let x of poi.times_per_person) {
    sum += x[1];
  }
  return sum / poi.times_per_person.length;
}

export let colours = ["#4C72B0", "#DD8452", "#55A868", "#C44E52", "#8172B3", "#937860", "#DA8BC3"]
