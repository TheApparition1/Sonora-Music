# Sonora Music

Sonora is a simple macOS native music player that is made using Tauri, Rust and SvelteKit. It is designed to let you play back local music files with ease.

## Tech Stack

- **Frontend**: SvelteKit 5 with Typescript and Tailwind CSS 4
- **Backend**: Rust with Tauri 2, Apple AVFoundation - Specifically AVAudioPlayer
- **Icons**: Lucide Svelte


## Development and Installation

- For end users, you can either download the app from the Sonora website or from the GitHub releases.

- For those who want to develop the app, run the commands below to set up your workspace.

```bash
cd sonora
npm install
npm run dev
```

## FAQ
- Why use Tauri?
  - I chose to use Tauri, as it gives me the simplicity of a native application, without needing to learn a new language.
  - Tauri allows you to use the same frontend languages that I already know, like SvelteKit and TailwindCSS.
  - It is also lighter than Electron and has a smaller footprint to save disk space.

- Why is this MacOS native?
  - The main reason is that I used Apple's AVFoundation systems to handle audio playback.
  - While I could use another framework and have a separate branch for Windows, it is a lot easier for me to focus on one app, especially as this is a side project.
  - AVAudioPlayer was also the easiest way to handle audio playback. I tried using Rodio, but there were too many issues, such as stuttering and glitching.

- Why did you choose to make this?
  - I am a DJ and Audio Engineer, and I also frequently edit videos. I needed a simple way to play back local music files, without the hassle of cloud sync.
  - I used Apple Music originally, but it was clunky and synced all local files to other devices, and there was no way I found to disable this.
  - Sonora solves all of these issues and unifies everything into a single and simplistic app with a beautiful and modern UI.
- Who are you?
  - I'm Samuel. I'm a Student and a hobbyist developer. I do full-stack development, primarily focusing on Next.js and SvelteKit.
  - I also make most of my apps in Tauri using Rust as a backend, as I have the most experience in those areas.
