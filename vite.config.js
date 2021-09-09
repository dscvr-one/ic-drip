import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";
import dfxJson from "./dfx.json";
import WindiCSS from 'vite-plugin-windicss'

let localCanisters, prodCanisters, canisters;

function initCanisterIds() {
  try {
    localCanisters = require(path.resolve(
      ".dfx",
      "local",
      "canister_ids.json"
    ));
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production");
  }
  try {
    prodCanisters = require(path.resolve("canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }

  const network = process.env.DFX_NETWORK || "local";

  canisters = network === "local" ? localCanisters : prodCanisters;

  for (const canister in canisters) {
    process.env["VITE_APP_" + canister.toUpperCase() + "_CANISTER_ID"] =
      canisters[canister][network];
  }
}
initCanisterIds();

// List of all aliases for canisters
const aliases = Object.entries(dfxJson.canisters).reduce(
  (acc, [name, _value]) => {
    // Get the network name, or `local` by default.
    const networkName = process.env["DFX_NETWORK"] || "local";
    const outputRoot = path.join(
      __dirname,
      ".dfx",
      networkName,
      "canisters",
      name
    );

    return {
      ...acc,
      ["dfx-generated/" + name]: path.join(outputRoot, name + ".did.js"),
    };
  },
  {}
);

//https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), WindiCSS()],
  resolve: {
    alias: {
      ...aliases,
    },
  },
  define: {
    "process.env": process.env,
  },
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8000",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, "/api"),
      },
    },
  },
});
