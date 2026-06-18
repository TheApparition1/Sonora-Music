<script lang="ts">
  import { Folder, Play, SkipBack, SkipForward } from 'lucide-svelte';
  
  let selectedFolder = $state("");

  function selectFolder() {
    console.log("Folder Selection Button Clicked");
  }
</script>

<div class="app">
  <div class="header">
    <h1>Sonora Music</h1>
  </div>

  <div class="main">
    <div class="folder-section">
      <button class="folder-btn" on:click={selectFolder}>
        <Folder size={20} />
        <span>Select Folder</span>
      </button>
      {#if selectedFolder}
        <p class="folder-path">{selectedFolder}</p>
      {/if}
    </div>

    <div class="library-placeholder">
      <p class="placeholder-text">No music loaded yet</p>
    </div>
  </div>

  <div class="player">
    <div class="track-info">
      <span class="track-name">No track playing</span>
    </div>

    <div class="controls">
      <button class="control-btn" disabled>
        <SkipBack size={20} />
      </button>
      <button class="control-btn play-btn" disabled>
        <Play size={24} />
      </button>
      <button class="control-btn" disabled>
        <SkipForward size={20} />
      </button>
    </div>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
  }

  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: #191724;
    color: #e0def4;
  }

  .header {
    padding: 1.25rem 2rem;
    border-bottom: 1px solid #26233a;
    background: #1f1d2e;
  }

  .header h1 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #c4a7e7;
  }

  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 2rem;
    gap: 1.5rem;
  }

  .folder-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .folder-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background: #26233a;
    border: 1px solid #31748f;
    border-radius: 6px;
    color: #9ccfd8;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.15s ease, border-color 0.15s ease;
    width: fit-content;
  }

  .folder-btn:hover {
    background: #31748f;
    border-color: #9ccfd8;
    color: #e0def4;
  }

  .folder-path {
    margin: 0;
    font-size: 0.875rem;
    color: #908caa;
  }

  .library-placeholder {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px dashed #26233a;
    border-radius: 8px;
    background: #1f1d2e;
  }

  .placeholder-text {
    color: #6e6a86;
    font-size: 1rem;
  }

  .player {
    padding: 1.25rem 2rem;
    background: #1f1d2e;
    border-top: 1px solid #26233a;
  }

  .track-info {
    margin-bottom: 1rem;
  }

  .track-name {
    font-size: 0.95rem;
    color: #e0def4;
    font-weight: 400;
  }

  .controls {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .control-btn {
    padding: 0.625rem 1rem;
    background: #26233a;
    border: 1px solid #31748f;
    border-radius: 6px;
    color: #9ccfd8;
    cursor: pointer;
    transition: background-color 0.15s ease, border-color 0.15s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .control-btn:hover:not(:disabled) {
    background: #31748f;
    border-color: #9ccfd8;
    color: #e0def4;
  }

  .control-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .play-btn {
    padding: 0.625rem 1.25rem;
  }
</style>
