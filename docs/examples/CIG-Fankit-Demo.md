# CIG Fankit Demo

This short demo page shows how to reference Fankit assets inside the repository (docs, UI, or examples).

## Logo example

![Star Citizen logo](../assets/fankit/03_LOGOS/STARCITIZEN_HORIZONTAL_BLACK.png)

Use path: `assets/fankit/03_LOGOS/STARCITIZEN_HORIZONTAL_BLACK.png`

## Wallpaper example

![Wallpaper sample](../assets/fankit/04_WALLPAPERS/4.0_Wallpapers/StarCitizen_4.0_2k_Wallpaper_01.jpg)

Use path: `assets/fankit/04_WALLPAPERS/4.0_Wallpapers/StarCitizen_4.0_2k_Wallpaper_01.jpg`

## Usage notes

- Always check `assets/fankit/LICENSE_CIG_FANKIT.txt` before using assets in documentation, in UI or for redistribution.
- When adding images to docs, prefer `STARCITIZEN_HORIZONTAL_BLACK.png` (transparent background) for headers and `StarCitizen_4.0_2k_Wallpaper_01.jpg` for demo backgrounds.

## Quick HTML snippet

```html
<!-- Example: include a fankit logo in a webpage -->
<img src="/assets/fankit/03_LOGOS/STARCITIZEN_HORIZONTAL_BLACK.png" alt="Star Citizen logo" style="max-width:320px;">
```

---
*This demo page is for internal documentation and CI testing; do not use assets in ways forbidden by the license.*