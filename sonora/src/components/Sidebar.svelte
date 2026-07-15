<script lang="ts">
  import { sidebarOpen, toggleSidebar } from '$lib/stores';
  import { Settings, X, Home, Music } from 'lucide-svelte';
</script>

{#if $sidebarOpen}
  <div class="sidebar-overlay" onclick={toggleSidebar}></div>
{/if}

<div class="sidebar" class:open={$sidebarOpen}>
  <div class="sidebar-header">
    <div class="logo">
      <Music size={24} />
      <span>Sonora</span>
    </div>
    <button class="close-btn" onclick={toggleSidebar}>
      <X size={20} />
    </button>
  </div>

  <nav class="sidebar-nav">
    <a href="/" class="nav-item" onclick={toggleSidebar}>
      <Home size={20} />
      <span>Player</span>
    </a>
    <a href="/settings" class="nav-item" onclick={toggleSidebar}>
      <Settings size={20} />
      <span>Settings</span>
    </a>
  </nav>
</div>

<style>
  .sidebar-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(2px);
    z-index: 998;
    animation: fadeIn 0.2s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .sidebar {
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    width: 300px;
    background: var(--header);
    border-right: 1px solid var(--border);
    z-index: 999;
    transform: translateX(-100%);
    transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    flex-direction: column;
    box-shadow: 4px 0 24px rgba(0, 0, 0, 0.3);
  }

  .sidebar.open {
    transform: translateX(0);
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem 1.5rem 1.25rem;
    border-bottom: 1px solid var(--border);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--accent);
  }

  .close-btn {
    background: var(--secondary);
    border: 1px solid var(--border);
    color: var(--muted);
    cursor: pointer;
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  .close-btn:hover {
    background: var(--button-hover);
    color: var(--text);
    border-color: var(--accent);
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 1rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    padding: 0.875rem 1rem;
    color: var(--muted);
    text-decoration: none;
    border-radius: 10px;
    transition: all 0.2s ease;
    font-weight: 500;
    font-size: 0.95rem;
  }

  .nav-item:hover {
    background: var(--secondary);
    color: var(--text);
  }

  .nav-item:global(.active) {
    background: var(--accent);
    color: var(--background);
  }
</style>
