# Inhibitor

Inhibits linux input devices. See the [Linux Kernel Docs][linux-docs] for more info.

## Motivation

The original use case was to use `inhibitor` + [systemd][systemd.device] to disable
my built-in keyboard whenever my bluetooth keyboard connects - something that I dearly
missed coming from MacOS's [karabiner-elements][karabiner], as I require a split
keyboard to avoid RSI. You can consult my exact [NixOS setup][nixos] if you're curious about
the details.

## License

Unless otherwise specified, all code in this repository is dual-licensed under
either:

- MIT-0 License ([LICENSE-MIT-0](LICENSE-MIT-0))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option. This means you can select the license you prefer!

Any contribution intentionally submitted for inclusion in the work by you, shall be
dual licensed as above, without any additional terms or conditions.

[linux-docs]: https://docs.kernel.org/input/input-programming.html#inhibiting-input-devices
[systemd.device]: https://www.freedesktop.org/software/systemd/man/latest/systemd.device.html
[karabiner]: https://karabiner-elements.pqrs.org/docs/manual/configuration/disable-built-in-keyboard/
[nixos]: https://github.com/TheButlah/nix/blob/e90227424661123437501f1bcdac5db47be04da3/machines/ryan-asahi/configuration.nix#L184-L215
