progress-rs
===========

`progress-rs` is a library to display a single progressbar and update it.

Contributing
------------

### using `nix`

After forking and cloning as usually you have to choose whether you want to use
bare nix or direnv.

* if you choose to use `direnv` copy the `.envrc.example` to `.envrc`, and adjust
  the `use flake` directive to `use nix` if you are not using flakes on your system.
  Be aware that not using flakes requires additional setup for the [rust overlay].
* if you are not using direnv use `nix develop`/`nix-shell` depending on your flake
  usage. If you are not using flakes you have to set up the [rust overlay] on your
  own.

[rust overlay]: https://github.com/oxalica/rust-overlay#installation

Then hack away as you'd usually with rust.

Pullrequests are only allowed agains the `develop` branch.
