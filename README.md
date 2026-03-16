<h1>Rust Rounds Clone</h1>
A Rust clone of the game [Rounds](https://store.steampowered.com/app/1557740/ROUNDS/), with a few additional features:
- Multiplayer for more than 2 players
- Performance and size improvements from using Rust and the [Bevy engine](https://bevy.org/) instead of Unity

<h2>Install Instructions</h2>
To ensure linux and windows compatibility, the game is developed in WSL2. 
Other operative systems are not tested but should be possible to support with minimal changes.

> [!NOTE]
> To quickly change target platform, set the PLATFORM variable with make:
> ```make build PLATFORM=linux```

<h3>Setup on Linux (WSL)</h3>

1. Setup steam
```sudo snap install steam```

2. Symlink steam to `~/.steam/`
```ln -s ~/snap/steam/common/.steam ~/.steam```

3. Run dev mode with steam running
```make dev```