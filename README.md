# derivepwd

derivepwd is a small CLI tool that derives passwords from a seed and a public component using a key derivation function.
You can use this tool to derive unique passwords for large fleets of devices.

It is written in Rust, consists of less than 100 lines of code and uses the excellent [ring](https://github.com/briansmith/ring) crate for cryptographic operations.

## Why?

Sometimes there are situations where you can't use public key schemes for authentication, for example for serial console access to devices.
Some of the requirements are:

- Have unique passwords per device
- High entropy but still typable
- Easy access to passwords when shit's on fire, yo
- Rotating the passwords is fast and easy (for example on offboardings, compromise, etc.)

Interfacing with enterprise grade password managers is slow and painful.
This tool tries to work around this problem by moving the per-device-part out of the password manager.

## How?

We mix a secret seed with a public part and shove them through a key derivation function.
The result is our password.

We use HKDF as key derivation function. It's fast and not memory hard, so **make sure your seed has enough entropy**.

The context is a concatenation from `hostname + "/" + role` and used for the info input of hkdf.
The use of `/` in the hostname is not allowed, to prevent collisions.

```
                                        seed key
                                            │
                                            ▼
  hostname ──┐     ┌────────┐ context ┌────────────┐
             ├────►│ concat ├────────►│    hkdf    ├─────► password
      role ──┘     └────────┘         └────────────┘
```

The generated passwords are 16 characters long, consisting of a set of 32 different characters.
To encode the passwords Derivepwd uses an alphanumeric character set that avoids similarly looking characters.
This results in 80 bits of entropy that are easy enough to type in a laggy VNC console with the wrong keyboard layout.

## Design Goals

- Easy to use
- Just one mode of operation
- Secure by default

## Usage

```
Usage: derivepwd [OPTIONS] <--seed <SEED>|--seed-file <SEED_FILE>> <HOSTNAME>

Arguments:
  <HOSTNAME>  hostname

Options:
  -s, --seed <SEED>            seed key as arugment
      --seed-file <SEED_FILE>  seed key as file
  -r, --role <ROLE>            role [default: root]
  -h, --help                   Print help
  -V, --version                Print version
```

### Example

```
# fetch seed from enterprise password manager
> SEED=$(op item get --vault network derivepwd-seed --fields password --format=json | jq -j .password)

# derive password for device
> derivepwd --seed=$SEED --role=ipmi router23.prod.example.com
zvhz2zshzbwdv43k
```

## Similar Projects

- [gokey](https://github.com/cloudflare/gokey)


## License

Copyright 2023 Wobcom GmbH

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
