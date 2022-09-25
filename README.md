# rusty-battery

[![crates.io](https://img.shields.io/crates/v/rusty-battery?logo=rust)](https://crates.io/crates/rusty-battery)
[![Test Suite](https://github.com/kucera-lukas/rusty-battery/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/kucera-lukas/rusty-battery/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/kucera-lukas/rusty-battery/branch/main/graph/badge.svg?token=1MM2CUE75Q)](https://codecov.io/gh/kucera-lukas/rusty-battery)

CLI tool to help you care about your devices's battery health.

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

Notify whenever battery percentage exceeds the given threshold.

USAGE:
`rusty-battery notify [FLAGS] [OPTIONS]`

FLAGS:

    -v, --verbose
            Activates verbose mode

OPTIONS:

    --kde-connect <kde-connect-names>...
            KDE Connect device names

            If this value is not present, KDE Connect will not be used.

            If this value is empty, all of the KDE Connect devices will be pinged.

    -m, --model <model>
            Battery model name

            If this value is omitted and only battery device is found for the current device, that one will be used.

            Otherwise, please use the `batteries` subcommand to get a list of all battery devices to get the model of
            the wanted battery device which should be monitored.

        --refresh-secs <refresh-secs>
            Number of seconds to wait before refreshing battery device data

            After every battery device refresh, its data will be checked. Notifications will be sent everytime they
            should be, based on the new refreshed battery device data. [default: 30]

    -t, --threshold <threshold>
            Battery charge threshold

            Whenever the chosen battery device reaches this charge threshold and will be charging, notifications will be
            sent, alerting that the charger should be unplugged. [default: 80]

## batteries

List all available batteries of the current device

USAGE:
`rusty-battery batteries [FLAGS]`

FLAGS:

    -v, --verbose    Activates verbose mode

## kde-connect-devices

List all available KDE Connect devices

USAGE:
`rusty-battery kde-connect-devices [FLAGS]`

FLAGS:

    -v, --verbose    Activates verbose mode

# Installation

## From [crates.io](https://crates.io/crates/rusty-battery)

`cargo install rust-battery`

## From [source](https://github.com/kucera-lukas/rusty-battery)

1. `git clone git@github.com:kucera-lukas/rusty-battery.git`
2. `cd rusty-battery`
3. `cargo install --path .`

## From [release page](https://github.com/kucera-lukas/rusty-battery/releases)

Download a binary of the
[latest release](https://github.com/kucera-lukas/rusty-battery/releases/latest)
and move it to a directory which is in your `$PATH`.
You may need to change the binary's permissions by running
`chmod +x rusty-battery`.

If there are any problems with the pre-compiled binaries, file an issue.

# Usage tips

This tool is best used when set up as a background task.

### Setup with `cron`

1. open terminal
2. `crontab -e` - this should open a text editor
3. paste in `@reboot rusty-battery notify [YOUR OPTIONS]`
4. save and exit the text editor, you should see `crontab: installing new crontab` in your terminal
5. `reboot`

### Logging

1. choose the log verbosity via the `-v` or `--verbose` flag
2. append it to the `rusty-battery` command
3. redirect output via `>> /path/to/log/file 2>&1`
4. check all logs via `more /path/to/log/file`

- for live logs use `tail`: `tail -f /path/to/log/file`
- `2>&1` [explanation](https://stackoverflow.com/questions/818255/in-the-shell-what-does-21-mean)

### Debugging

- [here is a useful thread](https://askubuntu.com/questions/23009/why-crontab-scripts-are-not-working) for crontab debugging
- to check that `rusty-battery` is running you can use `ps aux | grep -e rusty-battery`
- to kill the job you can use `kill $PID` (`$PID` can be found via the previous command)

# OS support

Currently, only linux is supported.
