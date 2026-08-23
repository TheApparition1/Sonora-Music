import { writable, derived } from 'svelte/store';
import { themes, defaultThemeId, type Theme } from './themes';

// Load theme from localStorage or use default
const storedThemeId = typeof window !== 'undefined' 
  ? localStorage.getItem('theme') || defaultThemeId 
  : defaultThemeId;

// Validate that stored theme still exists, otherwise use default
const validThemeId = themes.find(t => t.id === storedThemeId) ? storedThemeId : defaultThemeId;

export const currentThemeId = writable<string>(validThemeId);

export const currentTheme = derived(currentThemeId, ($id) => {
  return themes.find(t => t.id === $id) || themes[0];
});

// Update theme and save to localStorage
export function setTheme(themeId: string) {
  currentThemeId.set(themeId);
  if (typeof window !== 'undefined') {
    localStorage.setItem('theme', themeId);
  }
}

// Sidebar state
export const sidebarOpen = writable(false);

export function toggleSidebar() {
  sidebarOpen.update(n => !n);
}

// Player state
export const selectedFolder = writable<string>("");
export const musicFiles = writable<string[]>([]);
export const selectedTrack = writable<string | null>(null);
export const isPlaying = writable(false);
export const currentTrackIndex = writable(-1);
export const currentTime = writable(0);
export const duration = writable(0);
export const repeatMode = writable<'off' | 'all' | 'one'>('off');
export const shuffleMode = writable(false);
export const shuffledPlaylist = writable<string[]>([]);
export const originalPlaylist = writable<string[]>([]);