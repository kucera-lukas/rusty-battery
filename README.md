# rusty-battery

[![crates.io](https://img.shields.io/crates/v/rusty-battery?logo=rust)](https://crates.io/crates/rusty-battery)
[![Test Suite](https://github.com/kucera-lukas/rusty-battery/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/kucera-lukas/rusty-battery/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/kucera-lukas/rusty-battery/branch/main/graph/badge.svg?token=1MM2CUE75Q)](https://codecov.io/gh/kucera-lukas/rusty-battery)

CLI tool to help you care about your devices's battery health.

# Features

* [notify](#Notify)
* [batteries](#Batteries)
* [kde-connect-devices](#kde-connect-devices)

## notify

Notify whenever battery percentage exceeds the given threshold.

USAGE:
```rusty-battery notify [FLAGS] [OPTIONS]```

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
```rusty-battery batteries [FLAGS]```

FLAGS:

    -v, --verbose    Activates verbose mode

## kde-connect-devices

List all available KDE Connect devices

USAGE:
```rusty-battery kde-connect-devices [FLAGS]```

FLAGS:

    -v, --verbose    Activates verbose mode


# Installation

## From `crates.io`
``cargo install rust-battery``

## From source

1. `git clone git@github.com:kucera-lukas/rusty-battery.git`
2. `cd rusty-battery`
3. `cargo install --path .`


## From release page

Download a binary of the
[latest release](https://github.com/kucera-lukas/rusty-battery/releases/latest)
and move it to a directory which is in your `$PATH`.
You may need to change the binary's permissions by running
`chmod +x rusty-battery`.

If there are any problems with the pre-compiled binaries, file an issue.

## OS support

Currently, only linux is supported.
