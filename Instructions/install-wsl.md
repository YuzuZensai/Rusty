# Setting Up WSL2 and Integrating with VS Code

## Installing WSL2

Follow these steps to install WSL2 on your Windows machine:

1. Open a terminal with administrative privileges. To do this, right-click on the terminal icon and choose "Run as administrator".

2. Run the following command to start the WSL installation process:

```bash
wsl --install
```

3. After the installation is complete, reboot your computer.

4. If the Ubuntu console window doesn't automatically show up after reboot, follow these steps:
- Open a terminal with administrative privileges again.
- Click the plus icon on the tab bar of the terminal.
- Select "Ubuntu" from the options presented.

5. Once the Ubuntu installation process begins, you will be prompted to create a username and password for the Unix environment.

6. After completing the setup, you should now see the Linux console.

## Integrating WSL2 with VS Code

Now that you have WSL2 installed, you can integrate it with Visual Studio Code:

1. Open Visual Studio Code.

2. Go to the Extensions tab on the left sidebar and search for the "WSL" extension.

3. Install the "WSL" extension by clicking the "Install" button.

4. After the extension is installed, restart or reload Visual Studio Code to apply the changes.

5. You should now be able to use WSL2 as one of your available terminals in Visual Studio Code (The drop down arrow to the right of + icon around top right of your terminal section)

Now you're all set to work with WSL2 and use it seamlessly with Visual Studio Code!
