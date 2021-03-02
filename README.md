# valid8r

An open-source command line interface for linting your Ethereum 2.0 validator set up, maintained by Pondron LLC.

## Overview

valid8r is a tool to ensure Ethereum 2.0 validator set-up integrity to give peace of mind to the implementer and more importantly ensure the network runs smoothly. valid8r will acts as a linting tool and reads out to the implementer what system settings are correct, what systems settings need to be changed, and what system settings do not follow best practices.

## Installation

Download latest release from releases page: [Releases](https://github.com/pondron/valid8r/releases)

Currently supported - MacOS, Linux

untar:
```
$ tar -xzvf <PATH-TO-VALID8R>/valid8r-<version>-<arch>-<os>.tar.gz -C /usr/local/bin
```
ensure valid8r executable:
```
// if permission denied try ($ sudo chmod 755 /usr/local/bin/valid8r)
// if valid8r can't be found ensure /usr/local/bin is in path ($ echo $PATH)
$ valid8r --version
```

## `valid8r` usage

Valid8r takes two flags for parsing the Eth1 Client and Eth2 Client deployed on the local machine. (**CURRENTLY ONLY VALID8ING MAINNET**)

example:
```
// valid8r -1 <eth1client> -2 <eth2client>
$ valid8r -1 geth -2 lighthouse

// valid8r --eth1 <eth1client> --eth2 <eth2client>
$ valid8r --eth1 geth --eth2 lighthouse 
```

help/usage:
```
$ valid8r --help 
```

## Ethereum Client Support

| Client         |
| -------------- |
| geth           |
| besu           |
| nethermind     |
| openethereum   |

## Functionality: https://launchpad.ethereum.org/checklist
For proper functionality of valid8r please ensure client specific flags related to *--JsonRpc.Enabled* are enabled

**system requirements**
- ntp vs local time sync
- CPU/MEM/DISK capacity check

**network requirements**
- eth1 default ports 30303TCP/8545TCP
- eth2 default ports 9000TCP(prysm 13000TCP)
- if running as root check ssh agent is not running on port 22

**eth1 client**
- is running the latest version of the client
- is on mainnet
- can communicate with infura
- is in sync(visually crossreference block sync with infura above)
- current number of peers

## Tests

Unit tests will be included where applicable and can be run from the directory with

```
cargo test
```

## Issues

If you find an issue/bug or have a feature request please submit an issue here
[Issues](https://github.com/pondron/valid8r/issues)

## Contributing

If you are looking to contribute, please head to the
[Contributing](https://github.com/pondron/valid8r/blob/master/CONTRIBUTING.md) section.