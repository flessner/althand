<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";

  const window = getCurrentWindow();
  console.log(window);
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>
</main>
