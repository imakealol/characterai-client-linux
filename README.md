# characterai-client-linux (Debian/Kali, KDE/Wayland)

Native desktop client in Rust + GTK4.

## Build dependencies (Debian/Kali)

```
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libgtk-4-dev \
    libdbus-1-dev libsecret-1-dev
```

## Run

```
cargo run
```

## Getting your CharacterAI auth token

The app stores your token securely via the system keychain
(org.freedesktop.secrets – KWallet on KDE, GNOME Keyring on GNOME).

1. Open [character.ai](https://character.ai) in your browser and log in.
2. Open DevTools → Application → Cookies → `https://character.ai`.
3. Copy the value of `__Secure-next-auth.session-token`.
4. In the app, go to **Settings**, paste the token, and click **Save**.

The token is saved in the system keychain and loaded automatically on next launch.
To log out, click **Clear** in Settings.

## Usage

1. Open the **Settings** page and save your auth token.
2. Switch to the **Chat** page.
3. Enter a **Character ID** (visible in the character's page URL on character.ai,
   e.g. `https://character.ai/chat/CHAR_ID`).
4. Type a message and click **Send**.
