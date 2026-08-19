# Lofi Clouds Radio — Native Desktop App

Native desktop wrapper for `https://radio.lofi-clouds.com/#/` built with Tauri 2, React, and TypeScript.

## Repository

https://github.com/GFardad/lofi-clouds-native

## Verified state

- Frontend build: verified with `npm run build`
- GitHub repo: created and pushed
- CI: configured for Linux, Windows, and macOS via GitHub Actions
- Local native build in this environment: not yet verified due to network timeouts during `crates.io` index update during `tauri build`

## Development

```bash
npm install
npm run tauri:dev
```

## Build

```bash
npm run tauri:build
```

## Packaging targets

- Linux: `.deb`, AppImage, Flatpak
- Windows: `.msi`
- macOS: `.app`

## Notes

- App ID: `com.lofi-clouds.radio`
- Window style: overlay titlebar with embedded radio view
- The embedded playback surface uses a secure embedded browser view; if the live site later exposes a direct stream endpoint, the player path can be extended to native audio playback.
trigger ci
