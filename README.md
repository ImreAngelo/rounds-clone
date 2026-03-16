<h1>Rust Rounds Clone</h1>
<div>
    <p>
        A Rust clone of the game <a href="https://store.steampowered.com/app/1557740/ROUNDS/">Rounds</a>, 
        with a few additional features:
    </p>
    <ul>
        <li>Multiplayer for more than 2 players</li>
        <li>Performance and size improvements from using Rust and the <a href="https://bevy.org/">Bevy engine</a> instead of Unity</li>
    </ul>
</div>

<h2>Table of Contents</h2>
<ul>
    <li>
        <a href="#install">Install Instructions</a>
        <ul>
            <li><a href="#install-wsl">Setup on Linux (WSL)</a></li>
        </ul>
    </li>
</ul>

<h2 id="install">Install Instructions</h2>
<p>
To ensure linux and windows compatibility, the game is developed in WSL2. 
Other operative systems are not tested but should be possible to support with minimal changes.
</p>

> [!NOTE]
> To quickly change target platform, set the PLATFORM variable with make:
> ```make build PLATFORM=linux```

<h3 id="install-wsl">Setup on Linux (WSL)</h3>

1. Setup steam
```sudo snap install steam```

2. Symlink steam to `~/.steam/`
```ln -s ~/snap/steam/common/.steam ~/.steam```

3. Run dev mode with steam running
```make dev```