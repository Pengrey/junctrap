# junctrap

## Description

Tool aimed at fooling Chrome and users for cookies retrieval

The tool operates by creating a "shadow" user profile. It achieves this by:
1.  Creating a directory junction that points to Chrome's User Data directory.
2.  Modifying the Chrome shortcut in the taskbar to launch the browser with this new junction as the user data directory.

This process allows a user to log back into their accounts, creating new session tokens within this "shadow" profile. These new session tokens can then be exfiltrated. Due to what appears to be a bug, when the directory junction and modified shortcut are removed, these new sessions are merged with the original user profile.

## Features

*   **Trap:** Sets up the directory junction and modifies the Chrome shortcut.
*   **Dump:** Extracts session cookies from the shadow profile.
*   **Restore:** Removes the directory junction and restores the original Chrome shortcut.

## Usage

### Commands

**`trap`**

This command initiates the process. It creates a directory junction and modifies the Chrome shortcut.

```shell
junctrap.exe trap
```

**`dump`**

This command extracts the cookies from the shadow profile created by the `trap` command.

```shell
junctrap.exe dump
```

**`restore`**

This command cleans up the environment. It removes the directory junction and restores the original Chrome shortcut.

```shell
junctrap.exe restore
```

### Options

*   `--debug`: Enables debug logging for more verbose output.

## How It Works

The core of this tool's functionality lies in the manipulation of how Google Chrome loads a user's profile.

1.  **Directory Junction Creation:** A directory junction, which is a type of symbolic link on Windows, is created. This junction points to the original Google Chrome user data directory.
2.  **Shortcut Spoofing:** The tool then modifies the target of the Google Chrome shortcut in the user's taskbar. The modified shortcut points to the newly created directory junction. When the user launches Chrome from this shortcut, it uses the junction as its user data directory, effectively creating a separate "shadow" profile that still has access to the user's extensions and other settings. It also pads the command used with whitespaces so that the arguments are hidden from view.
3.  **Session Creation:** Because this shadow profile doesn't have the user's encrypted session tokens, the user is prompted to log in to their accounts again. This action creates new, session tokens within the shadow profile.
4.  **Cookie Dumping:** The `dump` command can then be used to extract these newly created session cookies from the shadow profile through the remote debugging protocol and due to the `user-data-dir` not being the default one it goes around current [mitigations](https://developer.chrome.com/blog/remote-debugging-port).
5.  **Restoration and Session Merging:** After the cookies are dumped, the `restore` command reverts the changes. It removes the directory junction and restores the original Chrome shortcut. Due to what seems to be a behavioral quirk in Chrome, the session created in the shadow profile is then merged with the original user profile, this can be seen bellow:
