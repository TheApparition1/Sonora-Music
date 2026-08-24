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
          <div class="theme-preview"><div class="color-swatch" style="background: {theme.colors.background}"></div><div class="color-swatch" style="background: {theme.colors.header}"></div><div class="color-swatch" style="background: {theme.colors.accent}"></div><div class="color-swatch" style="background: {theme.colors.buttonBorder}"></div></div>
          <span class="theme-name">{theme.name}</span>
          {#if $currentThemeId === theme.id}
            <span class="active-indicator">✓</span>
          {/if}
        </button>
      {/each}
    </div>
  </div>
<!--  EQ Section  -->

  <div class="settings-section">
    <h2>
      <Settings size={20} />
      EQ
    </h2>
    <div class="eq-section">
      <div class="eq-header">
        <h3>3 Band EQ</h3>
        <button class="toggle-btn active">On</button>
      </div>

      <div class="eq-bands">
        <div class="eq-band">
          <label>Low</label>
          <div class="slider-container">
            <input type="range" min="-6" max="6" step="0.5" value="0" />
          </div>
          <span class="gain-value">0dB</span>
        </div>

        <div class="eq-band">
          <label>Mid</label>
          <div class="slider-container">
            <input type="range" min="-6" max="6" step="0.5" value="0" />
          </div>
          <span class="gain-value">0dB</span>
        </div>

        <div class="eq-band">
          <label>High</label>
          <div class="slider-container">
            <input type="range" min="-6" max="6" step="0.5" value="0" />
          </div>
          <span class="gain-value">0dB</span>
        </div>
      </div>

      <button class="reset-btn">Reset EQ to Flat</button>
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
    display: table;
    width: 80px;
    height: 80px;
    margin: 0 auto;
    border-collapse: collapse;
    table-layout: fixed;
  }

  .color-swatch {
    display: table-cell;
    width: 40px;
    height: 40px;
    border-radius: 0;
    padding: 0;
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

/*  EQ Styling  */

  .eq-section {
    background: var(--secondary);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .eq-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
  }

  .eq-header h3 {
    font-size: 1.25rem;
    margin: 0;
    color: var(--text);
  }

  .toggle-btn {
    padding: 0.5rem 1rem;
    background: var(--button-bg);
    border: 1px solid var(--button-border);
    border-radius: 4px;
    color: var(--button-text);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .toggle-btn.active {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--background);
  }

  .eq-presets {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .presets-label {
    font-size: 0.95rem;
    color: var(--muted);
  }

  .preset-btn {
    padding: 0.4rem 0.8rem;
    background: var(--button-bg);
    border: 1px solid var(--button-border);
    border-radius: 4px;
    color: var(--button-text);
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .preset-btn:hover {
    background: var(--button-hover);
  }

  .preset-btn.active {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--background);
  }

  .eq-bands {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .eq-band {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .eq-band label {
    min-width: 40px;
    font-size: 0.9rem;
    color: var(--text);
    font-weight: 500;
  }

  .slider-container {
    flex: 1;
    position: relative;
  }

  .eq-band input[type="range"] {
    width: 100%;
    height: 6px;
    -webkit-appearance: none;
    background: var(--secondary);
    border-radius: 3px;
    cursor: pointer;
    border: 1px solid var(--border);
    box-shadow: 0 0 4px rgba(196, 167, 231, 0.1);
  }

  .eq-band input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 16px;
    height: 16px;
    background: var(--accent);
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 0 8px rgba(196, 167, 231, 0.4);
    border: 2px solid var(--background);
    margin-top: -5px;
  }

  .eq-band input[type="range"]::-webkit-slider-runnable-track {
    height: 6px;
    border-radius: 3px;
    background: var(--secondary);
  }

  .gain-value {
    min-width: 50px;
    text-align: right;
    font-size: 0.85rem;
    color: var(--muted);
    font-family: monospace;
  }

  .reset-btn {
    padding: 0.5rem 1rem;
    background: transparent;
    border: 1px solid var(--button-border);
    border-radius: 4px;
    color: var(--muted);
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .reset-btn:hover {
    border-color: var(--button-text);
    color: var(--text);
  }




</style>
