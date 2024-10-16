// @ts-check
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  site: "http://localhost:8000",
  output: "static",
  outDir: "../frontend-dist",
});
