# Forge Icons

This directory contains the application icons for Forge.

## Required Icon Sizes

Tauri requires the following icon files:

### macOS
- `icon.icns` - macOS app icon bundle

### Windows
- `icon.ico` - Windows icon file (includes 16x16, 32x32, 48x48, 256x256)

### Linux / General
- `32x32.png` - 32×32 pixels
- `128x128.png` - 128×128 pixels
- `128x128@2x.png` - 256×256 pixels (high DPI)
- `icon.png` - 512×512 pixels (main icon)

## Generating Icons

Use the included `icon.svg` as the source and generate all required sizes:

```bash
# Using ImageMagick
convert icon.svg -resize 32x32 32x32.png
convert icon.svg -resize 128x128 128x128.png
convert icon.svg -resize 256x256 128x128@2x.png
convert icon.svg -resize 512x512 icon.png

# Generate .ico for Windows
convert icon.svg -define icon:auto-resize=256,48,32,16 icon.ico

# Generate .icns for macOS (requires iconutil on macOS)
# First create an iconset folder with all required sizes, then:
# iconutil -c icns icon.iconset -o icon.icns
```

Or use the Tauri CLI:
```bash
pnpm tauri icon icon.svg
```

## Icon Design

The Forge icon represents:
- A stylized anvil (the "forge")
- A hammer striking the anvil
- Sparks representing productivity and creation
- Blue gradient representing trust and technology
