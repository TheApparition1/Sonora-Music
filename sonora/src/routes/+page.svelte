<script lang="ts">
  import { Folder, Play, SkipBack, SkipForward, Music, Pause, Repeat, Shuffle, Repeat1, Menu, ListMusic} from 'lucide-svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { toggleSidebar, selectedFolder, musicFiles, selectedTrack, isPlaying, currentTrackIndex, currentTime, duration, repeatMode, shuffleMode, shuffledPlaylist, originalPlaylist, currentViewTab } from '$lib/stores';
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
    
    if ($shuffleMode) {
      // Initialize shuffled playlist if empty
      if ($shuffledPlaylist.length === 0) {
        const shuffled = [...$musicFiles].sort(() => Math.random() - 0.5);
        shuffledPlaylist.set(shuffled);
      }
      currentTrackIndex.set($shuffledPlaylist.indexOf(file));
    } else {
      currentTrackIndex.set($musicFiles.indexOf(file));
    }
    
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
    
    // Ensure playlist is not empty
    if (playlist.length === 0) {
      isPlaying.set(false);
      return;
    }
    
    // Check if we're at the end of the playlist
    if ($currentTrackIndex >= playlist.length - 1) {
      if ($shuffleMode) {
        // Reshuffle and start from beginning
        const shuffled = [...$musicFiles].sort(() => Math.random() - 0.5);
        shuffledPlaylist.set(shuffled);
        currentTrackIndex.set(0);
        const nextTrack = shuffled[0];
        selectedTrack.set(nextTrack);
        if ($selectedFolder) {
          const fullPath = `${$selectedFolder}/${nextTrack}`;
          await invoke("play_music", { filePath: fullPath, index: 0, repeatMode: $repeatMode, shuffleMode: $shuffleMode });
          isPlaying.set(true);
        }
        return;
      } else if ($repeatMode === 'all') {
        // Start from beginning
        currentTrackIndex.set(0);
        const nextTrack = $musicFiles[0];
        selectedTrack.set(nextTrack);
        if ($selectedFolder) {
          const fullPath = `${$selectedFolder}/${nextTrack}`;
          await invoke("play_music", { filePath: fullPath, index: 0, repeatMode: $repeatMode, shuffleMode: $shuffleMode });
          isPlaying.set(true);
        }
        return;
      } else {
        // Stop playback
        await invoke("seek_to_time", { time: 0 });
        return;
      }
    }
    
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
    } else {
      // If Rust returns an error, try to handle it
      await invoke("seek_to_time", { time: 0 });
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
        // Use a more lenient check to ensure we catch the end of the song
        if (dur > 0 && time >= dur - 1 && $repeatMode !== 'one') {
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
      // Save original order
      originalPlaylist.set([...$musicFiles]);
      // Create shuffled playlist
      const shuffled = [...$musicFiles].sort(() => Math.random() - 0.5);
      // Move current track to position 0 if it exists
      if ($selectedTrack) {
        const currentIndex = shuffled.indexOf($selectedTrack);
        if (currentIndex !== -1) {
          [shuffled[0], shuffled[currentIndex]] = [shuffled[currentIndex], shuffled[0]];
        }
        currentTrackIndex.set(0);
      } else {
        currentTrackIndex.set(0);
      }
      shuffledPlaylist.set(shuffled);
    } else {
      // Clear shuffled playlist
      shuffledPlaylist.set([]);
      // Reset index to match original playlist
      if ($selectedTrack) {
        currentTrackIndex.set($musicFiles.indexOf($selectedTrack));
      }
    }
  }

  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }

  function removeFileExtension(filename: string): string {
    const lastDotIndex = filename.lastIndexOf('.');
    return lastDotIndex > 0 ? filename.substring(0, lastDotIndex) : filename;
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

    <div class="tabs">
      <button class="tab" class:active={$currentViewTab === 'library'} on:click={() => currentViewTab.set('library')}>
        <Music size={16} />
        <span>Library</span>
      </button>
      <button class="tab" class:active={$currentViewTab === 'queue'} on:click={() => currentViewTab.set('queue')}>
        <ListMusic size={16} />
        <span>Queue</span>
      </button>
    </div>

    <div class="library-placeholder">
      {#if $currentViewTab === 'library'}
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
      {:else if $currentViewTab === 'queue'}
        {#if $shuffleMode && $shuffledPlaylist.length > 0}
          <div class="music-list">
            {#each $shuffledPlaylist as file, index}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="music-item" class:selected={$selectedTrack === file} class:current={index === $currentTrackIndex} on:click={() => selectTrack(file)}>
                <span class="queue-number">{index + 1}</span>
                <Music size={16} />
                <span>{removeFileExtension(file)}</span>
              </div>
            {/each}
          </div>
        {:else if !$shuffleMode && $musicFiles.length > 0}
          <div class="music-list">
            {#each $musicFiles as file, index}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="music-item" class:selected={$selectedTrack === file} class:current={index === $currentTrackIndex} on:click={() => selectTrack(file)}>
                <span class="queue-number">{index + 1}</span>
                <Music size={16} />
                <span>{removeFileExtension(file)}</span>
              </div>
            {/each}
          </div>
        {:else}
          <p class="placeholder-text">Queue is empty</p>
        {/if}
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
    overflow: hidden;
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
    overflow-y: auto;
  }

  .folder-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: var(--secondary);
    border: 1px solid var(--border);
    border-radius: 8px;
    color: var(--muted);
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .tab:hover {
    background: var(--button-hover);
    color: var(--text);
  }

  .tab.active {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--background);
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

  .music-item.current {
    background: var(--accent);
    color: var(--background);
    border-color: var(--accent);
  }

  .queue-number {
    font-size: 0.75rem;
    color: var(--muted);
    font-weight: 600;
    min-width: 24px;
    text-align: center;
  }

  .music-item.current .queue-number {
    color: var(--background);
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
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }

  .track-name {
    font-size: 0.95rem;
    color: var(--text);
    font-weight: 400;
  }

  .debug-info {
    font-size: 0.75rem;
    color: var(--muted);
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
