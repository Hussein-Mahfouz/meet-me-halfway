import { MapModel } from "backend";
import type { Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export let sidebarContents: Writable<HTMLDivElement | null> = writable(null);
export let mapContents: Writable<HTMLDivElement | null> = writable(null);

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
  | { kind: "input" }
  | { kind: "results"; data: POI[] };

export let mode: Writable<Mode> = writable({ kind: "title" });
export let model: Writable<MapModel | null> = writable(null);
export let map: Writable<Map | null> = writable(null);
export let showAbout: Writable<boolean> = writable(true);
