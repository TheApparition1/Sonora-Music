<script lang="ts">
  import { Folder, Play, SkipBack, SkipForward, Music, Pause, Repeat, Shuffle, Repeat1, Menu} from 'lucide-svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { toggleSidebar, selectedFolder, musicFiles, selectedTrack, isPlaying, currentTrackIndex, currentTime, duration, repeatMode, shuffleMode, shuffledPlaylist, originalPlaylist } from '$lib/stores';
  import { onMount, onDestroy } from 'svelte';

  let progressInterval: number | null = null;


  async function selectFolder() {
    try {
      const folderPath = await invoke<string>("select_folder");
      selectedFolder.set(folderPath);
      
      const files = await invoke<string[]>("get_music_files", { folderPath });
      musicFiles.set(files);
    } catch (error) {
      console.error("Folder selection failed:", error);
    }
  }

  async function selectTrack(file: string) {
    selectedTrack.set(file);
    const playlist = $shuffleMode ? $shuffledPlaylist : $musicFiles;
    currentTrackIndex.set(playlist.indexOf(file));
    if ($selectedFolder) {
      const fullPath = `${$selectedFolder}/${file}`;
      try {
        await invoke("play_music", { filePath: fullPath, index: $currentTrackIndex, repeatMode: $repeatMode, shuffleMode: $shuffleMode });
        isPlaying.set(true);
        // Clear existing interval and start new progress updates
        if (progressInterval !== null) {
          clearInterval(progressInterval);
        }
        progressInterval = window.setInterval(updateProgress, 1000);
      } catch (error) {
        console.error("Failed to play music:", error);
      }
    }
  }

  async function togglePlayPause() {
    if ($isPlaying) {
      await invoke("pause_music");
      isPlaying.set(false);
    } else {
      await invoke("resume_music");
      isPlaying.set(true);
    }
  }

  async function skipNext() {
    const playlist = $shuffleMode ? $shuffledPlaylist : $musicFiles;
    const result = await invoke<number>("skip_next", { musicFiles: playlist, currentIndex: $currentTrackIndex, repeatMode: $repeatMode, shuffleMode: $shuffleMode });
    if (typeof result === "number") {
      currentTrackIndex.set(result);
      const nextTrack = playlist[result];
      selectedTrack.set(nextTrack);
      if ($selectedFolder) {
        const fullPath = `${$selectedFolder}/${nextTrack}`;
        await invoke("play_music", { filePath: fullPath, index: result, repeatMode: $repeatMode, shuffleMode: $shuffleMode });
        isPlaying.set(true);
      }
    }
  }

  async function skipPrevious() {
    const playlist = $shuffleMode ? $shuffledPlaylist : $musicFiles;
    const result = await invoke<number>("skip_previous", { musicFiles: playlist, currentIndex: $currentTrackIndex, repeatMode: $repeatMode, shuffleMode: $shuffleMode });
    if (typeof result === "number") {
      currentTrackIndex.set(result);
      const prevTrack = playlist[result];
      selectedTrack.set(prevTrack);
      if ($selectedFolder) {
        const fullPath = `${$selectedFolder}/${prevTrack}`;
        await invoke("play_music", { filePath: fullPath, index: result, repeatMode: $repeatMode, shuffleMode: $shuffleMode });
        isPlaying.set(true);
      }
    }
  }

  async function updateProgress() {
    if ($isPlaying) {
      try {
        const time = await invoke<number>("get_current_time");
        const dur = await invoke<number>("get_duration");
        currentTime.set(time);
        duration.set(dur);
        
        // Auto-advance to next track when song ends (unless repeat one)
        if (dur > 0 && time >= dur - 0.1 && $repeatMode !== 'one') {
          await skipNext();
        }
      } catch (error) {
        console.error("Failed to get progress:", error);
      }
    }
  }

  async function seekToTime(time: number) {
    try {
      await invoke("seek_to_time", { time });
      currentTime.set(time);
    } catch (error) {
      console.error("Failed to seek:", error);
    }
  }

  async function toggleRepeat() {
    let newMode: 'off' | 'all' | 'one';
    if ($repeatMode === 'off') {
      newMode = 'all';
    } else if ($repeatMode === 'all') {
      newMode = 'one';
    } else {
      newMode = 'off';
    }
    repeatMode.set(newMode);
    await invoke("set_repeat_mode", { repeatMode: newMode });
  }

  function toggleShuffle() {
    const newShuffleMode = !$shuffleMode;
    shuffleMode.set(newShuffleMode);
    if (newShuffleMode) {
      originalPlaylist.set([...$musicFiles]);
      const newShuffled = [...$musicFiles].sort(() => Math.random() - 0.5);
      const currentTrack = $musicFiles[$currentTrackIndex];
      const newIndex = newShuffled.indexOf(currentTrack);
      if (newIndex !== -1) {
        [newShuffled[0], newShuffled[newIndex]] = [newShuffled[newIndex], newShuffled[0]];
      }
      shuffledPlaylist.set(newShuffled);
    } else {
      shuffledPlaylist.set([]);
    }
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
    const newTime = percentage * $duration;
    seekToTime(newTime);
  }

  onDestroy(() => {
    if (progressInterval !== null) {
      clearInterval(progressInterval);
    }
  });
</script>

<div class="app">
  <div class="header">
    <button class="menu-btn" on:click={toggleSidebar}>
      <Menu size={24} />
    </button>
    <h1>Sonora Music</h1>
  </div>

  <div class="main">
    <div class="folder-section">
      <button class="folder-btn" on:click={selectFolder}>
        <Folder size={20} />
        <span>Select Folder</span>
      </button>
      {#if $selectedFolder}
        <p class="folder-path">{$selectedFolder}</p>
      {/if}
    </div>

    <div class="library-placeholder">
      {#if $musicFiles.length > 0}
        <div class="music-list">
          {#each $musicFiles as file}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="music-item" class:selected={$selectedTrack === file} on:click={() => selectTrack(file)}>
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
      <span class="track-name">{$selectedTrack || "No track playing"}</span>
    </div>

    <div class="progress-container">
      <span class="time-display">{formatTime($currentTime)}</span>
      <div class="progress-bar" on:click={seekToProgress}>
        <div class="progress-fill" style="width: {$duration > 0 ? ($currentTime / $duration) * 100 : 0}%"></div>
      </div>
      <span class="time-display">{formatTime($duration - $currentTime)}</span>
    </div>

    <div class="controls">
      <button class="control-btn" on:click={toggleShuffle} class:active={$shuffleMode} disabled={$currentTrackIndex === -1}>
        <Shuffle size={20} />
      </button>
      <div class="main-controls">
        <button class="control-btn" on:click={skipPrevious} disabled={$currentTrackIndex === -1}>
          <SkipBack size={20} />
        </button>
        <button class="control-btn play-btn" on:click={togglePlayPause} disabled={$currentTrackIndex === -1}>
          {#if $isPlaying}
            <Pause size={24} />
          {:else}
            <Play size={24} />
          {/if}
        </button>
        <button class="control-btn" on:click={skipNext} disabled={$currentTrackIndex === -1}>
          <SkipForward size={20} />
        </button>
      </div>
      <button class="control-btn" on:click={toggleRepeat} class:active={$repeatMode !== 'off'} disabled={$currentTrackIndex === -1}>
        {#if $repeatMode === 'one'}
          <Repeat1 size={20} />
        {:else}
          <Repeat size={20} />
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  :global(html) {
    margin: 0;
    padding: 0;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background: #191724;
    color: #e0def4;
  }

  .app {
    height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    background: var(--background);
    color: var(--text);
  }

  .header {
    padding: 1.25rem 2rem;
    border-bottom: 1px solid var(--border);
    background: var(--header);
    display: flex;
    align-items: center;
    gap: 1rem;
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

  .header h1 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--accent);
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
    background: var(--button-bg);
    border: 1px solid var(--button-border);
    border-radius: 6px;
    color: var(--button-text);
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.15s ease, border-color 0.15s ease;
    width: fit-content;
  }

  .folder-btn:hover {
    background: var(--button-hover);
    border-color: var(--button-text);
    color: var(--text);
  }

  .folder-path {
    margin: 0;
    font-size: 0.875rem;
    color: var(--muted);
  }

  .library-placeholder {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px dashed var(--border);
    border-radius: 8px;
    background: var(--header);
  }

  .placeholder-text {
    color: var(--muted);
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
    background: var(--secondary);
    border: 1px solid transparent;
    border-radius: 6px;
    color: var(--text);
    font-size: 0.9rem;
    transition: background-color 0.15s ease, border-color 0.3s ease-out, box-shadow 0.3s ease-out;
    cursor: pointer;
  }

  .music-item:hover {
    background: var(--secondary);
  }

  .music-item:active {
    background: var(--secondary);
  }

  .music-item.selected {
    background: var(--secondary);
    border: 1px solid var(--accent);
    box-shadow: 0 0 4px rgba(196, 167, 231, 0.3);
  }

  .player {
    padding: 1.25rem 2rem;
    background: var(--header);
    border-top: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    align-items: center;
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
    background: var(--secondary);
    border-radius: 2px;
    cursor: pointer;
    position: relative;
  }

  .progress-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
    transition: width 0.1s ease;
  }

  .time-display {
    font-size: 0.75rem;
    color: var(--muted);
    min-width: 40px;
    text-align: center;
  }

  .track-info {
    margin-bottom: 1rem;
  }

  .track-name {
    font-size: 0.95rem;
    color: var(--text);
    font-weight: 400;
  }

  .controls {
    display: flex;
    gap: 2rem;
    align-items: center;
    justify-content: center;
  }

  .main-controls {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .control-btn {
    padding: 0.625rem 1rem;
    background: var(--button-bg);
    border: 1px solid var(--button-border);
    border-radius: 6px;
    color: var(--button-text);
    cursor: pointer;
    transition: background-color 0.15s ease, border-color 0.15s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .control-btn:hover:not(:disabled) {
    background: var(--button-hover);
    border-color: var(--button-text);
    color: var(--text);
  }

  .control-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .play-btn {
    padding: 0.625rem 1.25rem;
  }

  .control-btn.active {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--background);
  }
</style>
