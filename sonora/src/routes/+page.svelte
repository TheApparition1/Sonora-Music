<script lang="ts">
  import { Folder, Play, SkipBack, SkipForward, Music, Pause, Volume2, VolumeX } from 'lucide-svelte';
  import { invoke } from "@tauri-apps/api/core";
  
  let selectedFolder = $state("");
  let musicFiles = $state<string[]>([]);
  let selectedTrack = $state<string | null>(null);
  let isPlaying = $state(false);
  let currentTrackIndex = $state(-1);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(1.0);
  let volumeSliderVisible = $state(false);
  let progressInterval: number | null = null;

  async function selectFolder() {
    try {
      const folderPath = await invoke<string>("select_folder");
      selectedFolder = folderPath;
      
      const files = await invoke<string[]>("get_music_files", { folderPath });
      musicFiles = files;
    } catch (error) {
      console.error("Folder selection failed:", error);
    }
  }

  async function selectTrack(file: string) {
    selectedTrack = file;
    currentTrackIndex = musicFiles.indexOf(file);
    if (selectedFolder) {
      const fullPath = `${selectedFolder}/${file}`;
      try {
        await invoke("play_music", { filePath: fullPath, index: currentTrackIndex });
        isPlaying = true;
        // Clear existing interval and start new progress updates
        if (progressInterval) clearInterval(progressInterval);
        progressInterval = setInterval(updateProgress, 1000);
      } catch (error) {
        console.error("Failed to play music:", error);
      }
    }
  }

  async function togglePlayPause() {
    if (isPlaying) {
      await invoke("pause_music");
      isPlaying = false;
    } else {
      await invoke("resume_music");
      isPlaying = true;
    }
  }

  async function skipNext() {
    const result = await invoke<number>("skip_next", { musicFiles });
    if (typeof result === "number") {
      currentTrackIndex = result;
      const nextTrack = musicFiles[result];
      selectedTrack = nextTrack;
      if (selectedFolder) {
        const fullPath = `${selectedFolder}/${nextTrack}`;
        await invoke("play_music", { filePath: fullPath, index: result });
        isPlaying = true;
        // Clear existing interval and start new progress updates
        if (progressInterval) clearInterval(progressInterval);
        progressInterval = setInterval(updateProgress, 1000);
      }
    }
  }

  async function skipPrevious() {
    const result = await invoke<number>("skip_previous", { musicFiles });
    if (typeof result === "number") {
      currentTrackIndex = result;
      const prevTrack = musicFiles[result];
      selectedTrack = prevTrack;
      if (selectedFolder) {
        const fullPath = `${selectedFolder}/${prevTrack}`;
        await invoke("play_music", { filePath: fullPath, index: result });
        isPlaying = true;
        // Clear existing interval and start new progress updates
        if (progressInterval) clearInterval(progressInterval);
        progressInterval = setInterval(updateProgress, 1000);
      }
    }
  }

  async function updateProgress() {
    if (isPlaying) {
      try {
        const time = await invoke<number>("get_current_time");
        const dur = await invoke<number>("get_duration");
        currentTime = time;
        duration = dur;
      } catch (error) {
        console.error("Failed to get progress:", error);
      }
    }
  }

  async function seekToTime(time: number) {
    try {
      await invoke("seek_to_time", { time });
      currentTime = time;
    } catch (error) {
      console.error("Failed to seek:", error);
    }
  }

  async function setVolume(vol: number) {
    try {
      await invoke("set_volume", { volume: vol });
      volume = vol;
    } catch (error) {
      console.error("Failed to set volume:", error);
    }
  }

  function toggleVolumeSlider() {
    volumeSliderVisible = !volumeSliderVisible;
  }

  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }

  function seekToProgress(e: MouseEvent) {
    const progressBar = e.currentTarget as HTMLElement;
    const rect = progressBar.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const percentage = clickX / rect.width;
    const newTime = percentage * duration;
    seekToTime(newTime);
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
      {#if musicFiles.length > 0}
        <div class="music-list">
          {#each musicFiles as file}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="music-item" class:selected={selectedTrack === file} on:click={() => selectTrack(file)}>
              <Music size={16} />
              <span>{file}</span>
            </div>
          {/each}
        </div>
      {:else}
        <p class="placeholder-text">No music loaded yet</p>
      {/if}
    </div>
  </div>

  <div class="player">
    <div class="track-info">
      <span class="track-name">{selectedTrack || "No track playing"}</span>
    </div>

    <div class="progress-container">
      <span class="time-display">{formatTime(currentTime)}</span>
      <div class="progress-bar" on:click={seekToProgress}>
        <div class="progress-fill" style="width: {duration > 0 ? (currentTime / duration) * 100 : 0}%"></div>
      </div>
      <span class="time-display">{formatTime(duration - currentTime)}</span>
    </div>

    <div class="controls">
      <button class="control-btn" on:click={skipPrevious} disabled={currentTrackIndex === -1}>
        <SkipBack size={20} />
      </button>
      <button class="control-btn play-btn" on:click={togglePlayPause} disabled={currentTrackIndex === -1}>
        {#if isPlaying}
          <Pause size={24} />
        {:else}
          <Play size={24} />
        {/if}
      </button>
      <button class="control-btn" on:click={skipNext} disabled={currentTrackIndex === -1}>
        <SkipForward size={20} />
      </button>
    </div>

    <div class="volume-wrapper">
      <div class="volume-icon" on:click={toggleVolumeSlider}>
        {#if volume > 0}
          <Volume2 size={20} />
        {:else}
          <VolumeX size={20} />
        {/if}
      </div>
      {#if volumeSliderVisible}
        <div class="volume-slider-container">
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={volume}
            on:input={(e) => setVolume(parseFloat(e.target.value))}
            class="volume-slider-vertical"
          />
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background: #191724;
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

  .music-list {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .music-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: #26233a;
    border: 1px solid transparent;
    border-radius: 6px;
    color: #e0def4;
    font-size: 0.9rem;
    transition: background-color 0.15s ease, border-color 0.3s ease-out, box-shadow 0.3s ease-out;
    cursor: pointer;
  }

  .music-item:hover {
    background: #26233a;
  }

  .music-item:active {
    background: #26233a;
  }

  .music-item.selected {
    background: #26233a;
    border: 1px solid #c4a7e7;
    box-shadow: 0 0 4px rgba(196, 167, 231, 0.3);
  }

  .player {
    padding: 1.25rem 2rem;
    background: #1f1d2e;
    border-top: 1px solid #26233a;
    display: flex;
    flex-direction: column;
    align-items: center;
    position: relative;
  }

  .progress-container {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 100%;
    max-width: 600px;
    margin-bottom: 1rem;
  }

  .progress-bar {
    flex: 1;
    height: 4px;
    background: #26233a;
    border-radius: 2px;
    cursor: pointer;
    position: relative;
  }

  .progress-fill {
    height: 100%;
    background: #c4a7e7;
    border-radius: 2px;
    transition: width 0.1s ease;
  }

  .time-display {
    font-size: 0.75rem;
    color: #988ba2;
    min-width: 40px;
    text-align: center;
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

  .volume-wrapper {
    display: flex;
    align-items: center;
    position: absolute;
    right: 1.5rem;
    bottom: 1.25rem;
  }

  .volume-icon {
    padding: 0.5rem;
    cursor: pointer;
    color: #ffffff;
    transition: color 0.15s ease;
  }

  .volume-icon:hover {
    color: #cccccc;
  }

  .volume-slider-container {
    position: absolute;
    bottom: 100%;
    right: 0;
    margin-bottom: 0.5rem;
    padding: 0.75rem 0.625rem;
    background: #1a1a1a;
    border-radius: 16px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    animation: flyOut 0.2s ease-out;
  }

  @keyframes flyOut {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .volume-slider-vertical {
    width: 100px;
    height: 4px;
    -webkit-appearance: none;
    appearance: none;
    background: #2a2a2a;
    border-radius: 2px;
    outline: none;
    cursor: pointer;
    transform: rotate(-90deg);
    accent-color: #ffffff;
  }

  .volume-slider-vertical::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    background: #ffffff;
    border-radius: 50%;
    cursor: pointer;
    border: none;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
    transition: transform 0.15s ease;
  }

  .volume-slider-vertical::-webkit-slider-thumb:hover {
    transform: scale(1.15);
  }

  .volume-slider-vertical::-webkit-slider-runnable-track {
    background: #2a2a2a;
    border-radius: 2px;
  }

  .volume-slider-vertical::-moz-range-thumb {
    width: 14px;
    height: 14px;
    background: #ffffff;
    border-radius: 50%;
    cursor: pointer;
    border: none;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
    transition: transform 0.15s ease;
  }

  .volume-slider-vertical::-moz-range-thumb:hover {
    transform: scale(1.15);
  }

  .volume-slider-vertical::-moz-range-track {
    background: #2a2a2a;
    border-radius: 2px;
  }
</style>
