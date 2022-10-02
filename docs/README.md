# rusty-battery

[![crates.io](https://img.shields.io/crates/v/rusty-battery?logo=rust)](https://crates.io/crates/rusty-battery)
[![pages-build-deployment](https://github.com/kucera-lukas/rusty-battery/actions/workflows/pages/pages-build-deployment/badge.svg)](https://rustybattery.lukaskucera.com)
[![Continuous Integration](https://github.com/kucera-lukas/rusty-battery/actions/workflows/ci.yml/badge.svg)](https://github.com/kucera-lukas/rusty-battery/actions/workflows/ci.yml)

CLI tool which notifies you when laptop battery reaches a threshold.

# Why should you use `rusty-battery`?

If you aren't able to set start/stop charge threshold
(for example via [TLP](https://linrunner.de/tlp/)) but would still like to
make sure that your battery won't exceed your preferred threshold.
`rusty-battery` can let you know when your battery reached the threshold by
showing a desktop notification and optionally pinging your
[KDE Connect](https://kdeconnect.kde.org/) devices.

# Features

- [notify](#notify)
- [batteries](#batteries)
- [kde-connect-devices](#kde-connect-devices)

## notify

Notify whenever battery percentage exceeds the given threshold

<ins>Usage:</ins> `rusty-battery notify [OPTIONS]`

<ins>Options:</ins>

    -t, --threshold <THRESHOLD>
            Battery charge threshold

            Whenever the chosen battery device reaches this charge threshold and will be charging, notifications will be sent, alerting that the charger should be unplugged.

            [minimum: 0] [maximum: 100]

            [default: 80]

    -v, --verbose...
            More output per occurrence

    -m, --model <MODEL>
            Battery model name

            If this value is omitted and only battery device is found for the current device, that one will be used.

            Otherwise, please use the `batteries` subcommand to get a list of all battery devices to get the model of the wanted battery device which should be monitored.

    -q, --quiet...
            Less output per occurrence

        --refresh-secs <REFRESH_SECS>
            Number of seconds to wait before refreshing battery device data

            After every battery device refresh, its data will be checked. Notifications will be sent everytime they should be, based on the new refreshed battery device data.

            [default: 30]

        --summary <SUMMARY>
            Notification summary

            Supported variables: THRESHOLD, CHARGE_STATE, MODEL, REFRESH_SECS

            Reference these variables in your summary like shell environment variables with the '$' prefix.

            [default: "Charge limit warning"]

        --body <BODY>
            Notification body

            Supported variables: THRESHOLD, CHARGE_STATE, MODEL, REFRESH_SECS

            Reference these variables in your body like shell environment variables with the '$' prefix.

            [default: "Battery percentage reached the $THRESHOLD% threshold, please unplug your charger"]

        --kde-connect [<KDE_CONNECT_NAMES>...]
            KDE Connect device names

            If this value is not present, KDE Connect will not be used.

            If this value is empty, all of the KDE Connect devices will be pinged.

        --disable-desktop
            Disable desktop notifications

            Specify this flag if you don't want desktop notifications to be shown whenever the chosen battery percentage exceeds the given threshold.

    -h, --help
            Print help information (use `-h` for a summary)

    -V, --version
            Print version information

## batteries

List all available batteries of the current device

<ins>Usage:</ins> `rusty-battery batteries [OPTIONS]`

<ins>Options:</ins>

    -h, --help       Print help information
    -q, --quiet      Less output per occurrence
    -v, --verbose    More output per occurrence
    -V, --version    Print version information

## kde-connect-devices

List all available KDE Connect devices

<ins>Usage:</ins> `rusty-battery kde-connect-devices [OPTIONS]`

<ins>Options:</ins>

    -h, --help       Print help information
    -q, --quiet      Less output per occurrence
    -v, --verbose    More output per occurrence
    -V, --version    Print version information

# Installation

## From [crates.io](https://crates.io/crates/rusty-battery)

```sh
cargo install rusty-battery
```

## From [source](https://github.com/kucera-lukas/rusty-battery)

1. Clone the repository

```sh
git clone git@github.com:kucera-lukas/rusty-battery.git
```

2. Change directory

```sh
cd rusty-battery
```

3. Install with cargo

```sh
cargo install --path .
```

## From [release page](https://github.com/kucera-lukas/rusty-battery/releases)

Download a binary of the
[latest release](https://github.com/kucera-lukas/rusty-battery/releases/latest)
and move it to a directory which is in your `$PATH`.
You may need to change the binary's permissions by running:

```sh
chmod +x rusty-battery
```

If there are any problems with the pre-compiled binaries, file an issue.

# Usage tips

This tool is best used when set up as a background task.

### Setup with `cron`

1. Open crontab

```sh
crontab -e
```

2. Paste in `@reboot rusty-battery notify [YOUR OPTIONS]`
3. Save and exit the text editor, you should
   see `crontab: installing new crontab` in your terminal
4. Reboot the system

```sh
reboot
```

### Logging

1. Choose the log verbosity via the `-v` or `--verbose` flag
2. Append it to the `rusty-battery` command
3. Redirect output via `>> /path/to/log/file 2>&1`
4. Check all logs via

```sh
more /path/to/log/file
```

- Check live logs:

```sh
tail -f /path/to/log/file
```

- `2>&1` [explanation](https://stackoverflow.com/questions/818255/in-the-shell-what-does-21-mean)

### Debugging

- [Here is a useful thread](https://askubuntu.com/questions/23009/why-crontab-scripts-are-not-working)
  for crontab debugging
- To check that `rusty-battery` is running you can use

```sh
ps aux | grep -e rusty-battery
```

- To kill the job you can use (`$PID` can be found via the previous command):

```sh
kill $PID
```

# Device support

Tested on:

- OS: Fedora 34, 35, 36
- DE: Plasma
