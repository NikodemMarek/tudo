# tudo

Simple tui wrapper for google-tasks1 rust library.
Manage your google tasks from the terminal.

### Installation

Install tudo with cargo.

```sh
cargo install --git https://github.com/NikodemMarek/tudo.git
```

Create config `tudo` folder in your config dir,
and create config files:

config.toml
```toml
client_secret = "/<path-to-config-dir>/tudo/client_secret.json"
```

client_secret.json
```json
{
    "installed": {
        "client_id": "",
        "client_secret": "",
        "token_uri": "https://oauth2.googleapis.com/token",
        "auth_uri": "https://accounts.google.com/o/oauth2/v2/auth",
        "redirect_uris": ["http://localhost:6555/"]
    }
}
```

