<div align = "center">
  <img src = "../assets/header.png" alt = "header" />
  <br />
  <a href = "https://github.com/lencydev/valorant-triggerbot/releases/latest"><img src = "https://img.shields.io/github/v/release/lencydev/valorant-triggerbot?style=flat-square&color=5864F9" alt = "Release" /></a>
  <a href = "https://github.com/lencydev/valorant-triggerbot/blob/main/LICENSE"><img src = "https://img.shields.io/github/license/lencydev/valorant-triggerbot?style=flat-square&color=5864F9" alt = "License" /></a>
  <a href = "https://github.com/lencydev/valorant-triggerbot/releases"><img src = "https://img.shields.io/github/downloads/lencydev/valorant-triggerbot/total?style=flat-square&color=5864F9" alt = "Downloads" /></a>
  <a href = "https://github.com/lencydev/valorant-triggerbot/forks?include=active,archived,inactive,network&page=1&period=&sort_by=last_updated"><img src = "https://img.shields.io/github/forks/lencydev/valorant-triggerbot?style=flat-square&color=5864F9" alt = "Forks" /></a>
  <a href = "https://github.com/lencydev/valorant-triggerbot/stargazers"><img src = "https://img.shields.io/github/stars/lencydev/valorant-triggerbot?style=flat-square&color=5864F9" alt = "Stars" /></a>
  <br />
  <a href = "https://ko-fi.com/lencydev"><img src = "https://img.shields.io/badge/Ko--fi-FF6433?logo=ko-fi&logoColor=FFF&style=flat-square" alt = "Ko-fi" /></a>
</div>

## Features
- **Color Detection:** Automatically fires when the configured enemy highlight color is detected in the trigger area.
- **Customizable Trigger Area:** Define the size of the detection area at the center of your screen.
- **Adjustable Trigger Delay:** Set a delay (in milliseconds) before firing after color detection.
- **Configurable Trigger Keys:** Choose specific keyboard keys or mouse buttons to activate the triggerbot (hold mode).
- **Color Tolerance:** Adjust the sensitivity of color detection to account for variations in lighting or effects.
- **Resolution Setting:** Configure the application based on your in-game screen resolution.
- **Simple GUI:** Easy-to-use interface for managing settings.

# Usage
1. Download the latest release from the [releases](https://github.com/lencydev/valorant-triggerbot/releases) page.
2. Run the `valorant-triggerbot.exe` file.
3. Configure the settings in the application to match your preferences and in-game settings.
4. Click the `Enable` button to activate the triggerbot.

## Configuration
Use the application's graphical interface to adjust the following settings:

- **Resolution:** Set this to match your Valorant game resolution (e.g. 1920x1080).
- **Trigger Keys:** Select the keys/buttons you want to hold to activate the triggerbot (only active if `Always Open` is off).
- **Trigger Delay:** The time in milliseconds the bot waits after detecting the color before firing. Increase if it fires too early, decrease if too late.
- **Trigger Area:** The size (in pixels) of the square area in the center of the screen where the bot looks for the target color.
- **Target Color:** The RGB values of the enemy highlight color. Default is Purple (R 240 G 90 B 255).
- **Color Tolerance:** How much variation from the Target Color is allowed. Higher values are more lenient but might cause false positives. Lower values are stricter.

### Valorant Settings
These are the in-game settings required for the triggerbot to work correctly:
- `General > Accessibility > Enemy Highlight Color:` Select `Purple` (or match the Target Color configured in the bot).
- `Controls > Equipment > Weapons > Fire:` Set the secondary keybind to `K` (the bot simulates pressing `K`).

### Default Settings
These are the default and recommended starting settings.
Adjust them based on your needs.
- `Resolution:` 1920x1080
- `Trigger Keys:` Left Shift
- `Trigger Delay:` 50ms
- `Trigger Area:` 5.0
- `Target Color:` Purple (R 240 G 90 B 255)
- `Color Tolerance:` 50

# Building from Source
If you prefer to build the application yourself, follow these steps:

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Clone the repository:

   ```bash
   git clone https://github.com/lencydev/valorant-triggerbot.git
   cd valorant-triggerbot
   ```
3. Build the project:

   ```bash
   cargo build --release
   ```
4. After the compilation:
   - Navigate to the `target/release` directory.
   - Find the `valorant-triggerbot.exe` file.
   - Run the executable to start the application.

# Disclaimer
> [!WARNING]
> Using third-party applications that interact with the game, like this triggerbot,<br />
> is against Riot Games' Terms of Service and can lead to account suspension or permanent ban.<br />
> Use this software entirely at your own risk.

- While efforts may be made to avoid detection, anti-cheat systems like Vanguard are constantly updated, and detection is always a possibility.
- The author is not responsible for any bans or other consequences resulting from the use of this software.

# Support
If you encounter issues or have questions, you can:
- Check the [Troubleshooting](#troubleshooting) and [FAQ](#faq) sections below.
- Message me on [discord](https://discord.com/users/313738210729656332).

## Troubleshooting
- **Triggerbot not firing:**
    - Ensure Valorant's `Enemy Highlight Color` matches the bot's `Target Color`.
    - Verify the secondary fire keybind in Valorant is set to `K`.
    - Check if the `Resolution` setting in the bot matches your game resolution.
    - Make sure the bot is `Enabled`.
    - Try increasing `Color Tolerance` slightly.
    - Ensure the trigger area is positioned correctly over your crosshair.

## FAQ
- **Q: Is this safe to use? Will I get banned?**
  - **A:** Using any third-party tool that gives an unfair advantage violates Valorant's ToS and carries a high risk of banning. Use at your own discretion. See the [Disclaimer](#disclaimer).
- **Q: Does it work with different enemy highlight colors?**
  - **A:** Yes, but you need to manually configure the correct RGB values in the `Target Color` setting within the triggerbot application.
- **Q: Does it work on all screen resolutions?**
  - **A:** It should work if you correctly set your game's resolution in the triggerbot's `Resolution` setting.
- **Q: Can I change the key the bot presses to fire?**
  - **A:** Currently, the bot is hardcoded to press `K`. You need to set `K` as your secondary fire keybind in Valorant. Modifying this would require changing the source code.

## How it Works
The triggerbot operates on a simple principle:
1. It continuously captures a small portion of your screen defined by the `Trigger Area` setting, centered around where your crosshair typically is.
2. It analyzes the pixels within this captured area.
3. If it finds pixels matching the configured `Target Color` (within the specified `Color Tolerance`), it assumes an enemy is under the crosshair.
4. After waiting for the configured `Trigger Delay`, it simulates a key press (`K` by default) to make the game fire.

<hr />

<div align = "center">
  <img src = "https://github.com/user-attachments/assets/98ce95a5-3fde-4bdb-b9b1-54825743a556" alt = "GUI" />
</div>

<hr />

<div align = "center">
  <video src = "https://github.com/user-attachments/assets/c2a3a180-24ae-4ed2-9c6d-b27732631dc2"></video>
</div>
