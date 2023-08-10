# Setting up MSYS2 and Integrating it with VS Code

## Installing MSYS2
1. Download MSYS2 from [https://www.msys2.org/](https://www.msys2.org/).
2. Run the downloaded installer, and if prompted, allow administrative permissions.
3. Choose the installation folder. Unless you have specific requirements, it's recommended to use the default installation path.
4. Leave the start menu shortcut as the default or adjust it according to your preference.
5. Wait for the installation process to complete.

## Integrating MSYS2 with VS Code
1. Open Visual Studio Code.
2. Go to **File** > **Preferences** > **Settings** or simply press `Ctrl` + `,`.
3. In the search bar at the top, search for `terminal.integrated.profiles.windows` and click on the "Edit in settings.json" link.

4. Add the following configuration to the `settings.json` file:

```json
"terminal.integrated.profiles.windows": {
  "MSYS2 Bash": {
    "path": "C:\\msys64\\usr\\bin\\bash.exe",
    "args": ["--login", "-i"],
    "env": {
      "MSYSTEM": "MINGW64",
      "CHERE_INVOKING": "1",
      "MSYS2_PATH_TYPE": "inherit"
    }
  }
}
```

## Here's how to access it:

1. Open Visual Studio Code.
2. In the terminal section, located at the top right of the window, you'll see a dropdown arrow to the right of the "+" icon.
3. Click on the dropdown arrow to reveal the list of available terminals.
4. You'll find MSYS2 Bash listed there. Click on it to open a MSYS2 Bash terminal within VS Code.
