# FoxBook setup

This is the README for an exectuable that outputs shell scripts that can be used to setup a laptop to be immediately booted into 
firefox. The resulting machine will be able to do little else without installing extra stuff, but that will be possible,
and firefox may be all you need.

## Building this exectuable

Install `rust` and `cargo` via [rustup.rs](https://rustup.rs/).

We also need to install the musl target. So you need to run 

```
rustup target add x86_64-unknown-linux-musl
```

to install it, then run

```
cargo build --target x86_64-unknown-linux-musl --release
```

to build it. The binary will be in the `target/x86_64-unknown-linux-musl/release` folder.
These musl instructions were based on [this page](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html)

## The Setup Procedure

The steps below have scripts associated with them. The example commands to run them assume that you have named the 
executable `foxbook-setup` and that your desired username is `myuser`. Replace all instances of those strings with 
different values as desired.

### Step 0
The first step is to install alpine linux. This set of instructions was developed using version 3.11.3 and installing 
with a 'sys' configuration (actually installing to a disk) but other future versions and configs will probably broadly
work. So if you've done that, you should be logged in as root, booted ofthe media you plan to continue to use going
forward, with any installation media removed.

As of now, it seems like `wpa supplicant` just isn't setup to start automatically after an install.
See https://gitlab.alpinelinux.org/alpine/aports/issues/8025
So, you may need to run the following to get an internet connection in order to download the exe itself to continue 
with these instructions:

```
rc-service wpa_supplicant start
```

This script performs miscellaneous setup that is easier to perform as root, including attempting to make sure wireless
internet access will work on the next boot. This script also prompts you to add a user with the name you provide.

```
foxbook-setup 0 myuser | ash
```

### Step 1

This script elevates the permissions of the user with the name based on the second parameter.

```
foxbook-setup 1 myuser | ash
```

Now you should be in your new account's home directory, and you can proceed to the next step.

### Step 2

This script prompts you to run the built-in xorg setup script.

```
foxbook-setup 2 | ash
```

### Step 3

This script installs and configures x11 and the i3 window manager. After running the script a graphical environment 
should appear and you should be prompted to setup an i3wm config. The defaults should work so you can just press enter
throughout that. After that is done you will be presented with a blank screen and you will need to either reboot or 
shutdown and restart the machine.

```
foxbook-setup 3 | ash
```

### Step 4

This script will set Firefox to startup automatically on boot. You will need to either reboot or shutdown and restart
the machine once more, but after this it should all work as expected.

```
foxbook-setup 4 | ash
```
