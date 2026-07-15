export interface Theme {
  name: string;
  id: string;
  colors: {
    background: string;
    header: string;
    secondary: string;
    text: string;
    muted: string;
    accent: string;
    border: string;
    buttonBg: string;
    buttonBorder: string;
    buttonText: string;
    buttonHover: string;
  };
}

export const themes: Theme[] = [
  {
    name: 'Rose Pine',
    id: 'rose-pine',
    colors: {
      background: '#191724',
      header: '#1f1d2e',
      secondary: '#26233a',
      text: '#e0def4',
      muted: '#908caa',
      accent: '#c4a7e7',
      border: '#26233a',
      buttonBg: '#26233a',
      buttonBorder: '#31748f',
      buttonText: '#9ccfd8',
      buttonHover: '#31748f',
    },
  },
  {
    name: 'Ocean Blue',
    id: 'ocean-blue',
    colors: {
      background: '#0f172a',
      header: '#1e293b',
      secondary: '#334155',
      text: '#f1f5f9',
      muted: '#94a3b8',
      accent: '#38bdf8',
      border: '#334155',
      buttonBg: '#334155',
      buttonBorder: '#0ea5e9',
      buttonText: '#7dd3fc',
      buttonHover: '#0ea5e9',
    },
  },
  {
    name: 'Forest Green',
    id: 'forest-green',
    colors: {
      background: '#1a2e1a',
      header: '#243424',
      secondary: '#2d4a2d',
      text: '#e8f5e8',
      muted: '#8fa88f',
      accent: '#4ade80',
      border: '#2d4a2d',
      buttonBg: '#2d4a2d',
      buttonBorder: '#22c55e',
      buttonText: '#86efac',
      buttonHover: '#22c55e',
    },
  },
  {
    name: 'Sunset Orange',
    id: 'sunset-orange',
    colors: {
      background: '#2a1f1a',
      header: '#3d2b24',
      secondary: '#4a372e',
      text: '#f5e8e0',
      muted: '#a88f8f',
      accent: '#fb923c',
      border: '#4a372e',
      buttonBg: '#4a372e',
      buttonBorder: '#ea580c',
      buttonText: '#fdba74',
      buttonHover: '#ea580c',
    },
  },
  {
    name: 'Midnight Purple',
    id: 'midnight-purple',
    colors: {
      background: '#1a1625',
      header: '#241f34',
      secondary: '#2d2642',
      text: '#f0e8f5',
      muted: '#a68fa8',
      accent: '#a855f7',
      border: '#2d2642',
      buttonBg: '#2d2642',
      buttonBorder: '#9333ea',
      buttonText: '#d8b4fe',
      buttonHover: '#9333ea',
    },
  },
];

export const defaultThemeId = 'rose-pine';
