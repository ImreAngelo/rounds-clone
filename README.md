## Install Instructions


> [!NOTE]
> To quickly change target platform, set the PLATFORM variable with make:
> ```make build PLATFORM=linux```

### Setup on Linux (WSL)
1. Setup steam
`sudo snap install steam`

2. Symlink steam to `~/.steam/`
`ln -s ~/snap/steam/common/.steam ~/.steam`
