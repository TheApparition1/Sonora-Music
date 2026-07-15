<script lang="ts">
  import { currentTheme } from '$lib/stores';
  import Sidebar from '../components/Sidebar.svelte';
  import { onMount, onDestroy } from 'svelte';

  let unsubscribe: () => void;

  function applyTheme() {
    const theme = $currentTheme;
    if (theme) {
      document.documentElement.style.setProperty('--background', theme.colors.background);
      document.documentElement.style.setProperty('--header', theme.colors.header);
      document.documentElement.style.setProperty('--secondary', theme.colors.secondary);
      document.documentElement.style.setProperty('--text', theme.colors.text);
      document.documentElement.style.setProperty('--muted', theme.colors.muted);
      document.documentElement.style.setProperty('--accent', theme.colors.accent);
      document.documentElement.style.setProperty('--border', theme.colors.border);
      document.documentElement.style.setProperty('--button-bg', theme.colors.buttonBg);
      document.documentElement.style.setProperty('--button-border', theme.colors.buttonBorder);
      document.documentElement.style.setProperty('--button-text', theme.colors.buttonText);
      document.documentElement.style.setProperty('--button-hover', theme.colors.buttonHover);
    }
  }

  onMount(() => {
    applyTheme();
    unsubscribe = currentTheme.subscribe(applyTheme);
  });

  onDestroy(() => {
    if (unsubscribe) unsubscribe();
  });
</script>

<Sidebar />

<slot />

<style>
  :global(html) {
    margin: 0;
    padding: 0;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background: var(--background);
    color: var(--text);
  }
</style>
