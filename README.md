# Nvidia wayland fancurve
This project aims to control fan speed on Nvidia GPU under Wayland session.

## Requirements
- Nvidia closed source driver
- [nvidia-setting](https://archlinux.org/packages/extra/x86_64/nvidia-settings/)
- [xorg-xhost](https://archlinux.org/packages/extra/x86_64/xorg-xhost/)
- sudo with NOPASSWD

## Instalation
- Build with ```cargo build --release```
- Make sure that user that runs this has ability to use `sudo` without password
- Put config file into `~/.config/nvidia-fancurve/config.json` (example is in `config/nvidia-fancurve/config.json`)
- Run app `./target/release/nvidia-fancurve`

## TODO
- Integration with sudo to support PASSWD option
- AUR package
- NIX package?
- SystemD service
