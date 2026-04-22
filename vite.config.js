import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    strictPort: true,
    host: true, // Listen on all network interfaces
    hmr: {
      protocol: "ws", // Use plain WS, not WSS
      host: "localhost", // Use localhost so adb reverse catches it
      port: 3000, // Match the server port
    },
  },
});
