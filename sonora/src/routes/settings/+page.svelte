<script lang="ts">
  import { currentThemeId, currentTheme, setTheme, toggleSidebar } from '$lib/stores';
  import { themes } from '$lib/themes';
  import { Settings, Palette, Menu } from 'lucide-svelte';
</script>

<div class="settings-container">
  <div class="settings-header">
    <button class="menu-btn" on:click={toggleSidebar}>
      <Menu size={24} />
    </button>
    <Settings size={24} />
    <h1>Settings</h1>
  </div>

  <div class="settings-section">
    <h2>
      <Palette size={20} />
      Theme
    </h2>
    <div class="theme-grid">
      {#each themes as theme}
        <button
          class="theme-card"
          class:active={$currentThemeId === theme.id}
          on:click={() => setTheme(theme.id)}
        >
          <div class="theme-preview">
            <div class="color-swatch" style="background: {theme.colors.background}"></div>
            <div class="color-swatch" style="background: {theme.colors.header}"></div>
            <div class="color-swatch" style="background: {theme.colors.accent}"></div>
            <div class="color-swatch" style="background: {theme.colors.buttonBorder}"></div>
          </div>
          <span class="theme-name">{theme.name}</span>
          {#if $currentThemeId === theme.id}
            <span class="active-indicator">✓</span>
          {/if}
        </button>
      {/each}
    </div>
  </div>
</div>

<style>
  .settings-container {
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
  }

  .settings-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--border);
  }

  .menu-btn {
    background: transparent;
    border: none;
    color: var(--muted);
    cursor: pointer;
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 6px;
    transition: background-color 0.2s ease;
  }

  .menu-btn:hover {
    background: var(--secondary);
    color: var(--text);
  }

  .settings-header h1 {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 600;
    color: var(--text);
  }

  .settings-section {
    margin-bottom: 2rem;
  }

  .settings-section h2 {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0 0 1.5rem 0;
    font-size: 1.25rem;
    font-weight: 500;
    color: var(--text);
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
  }

  .theme-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    padding: 1.5rem;
    background: var(--secondary);
    border: 2px solid var(--border);
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
  }

  .theme-card:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
  }

  .theme-card.active {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px rgba(196, 167, 231, 0.2);
  }

  .theme-preview {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.25rem;
    width: 100%;
  }

  .color-swatch {
    aspect-ratio: 1;
    border-radius: 4px;
    height: 12px;
  }

  .theme-name {
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--text);
  }

  .active-indicator {
    position: absolute;
    top: 0.75rem;
    right: 0.75rem;
    font-size: 1rem;
    color: var(--accent);
    font-weight: bold;
  }
</style>
