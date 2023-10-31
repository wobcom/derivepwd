# derivepwd

derivepwd is a small CLI tool that derives passwords from a seed and a public component using a key derivation function.

You can use this tool to generate unique passwords for large fleets of devices.

## Why?

Sometimes there are situations where you can't use public key schemes for authentication, for example for serial console access to devices.
Some of the requirements are:

- Have unique passwords per device
- Easy access to passwords when shit's on fire, yo
- Rotating the passwords is fast and easy (for example on offboardings)

Interfacing with enterprise grade password managers is slow and painful.
This tool tries to work around this problem by moving the per device part out of the password manager.

## How?

We mix a secret seed with a public part and shove them through a key derivation function.
The result is our password.

We use the HKDF as a key derivation function, so **make sure to use high enough entropy seed**.

```
                                          seed key
                                              │
                                              ▼
  hostname ──┐      ┌─────────┐        ┌────────────┐
             ├──────► context ├───────►│    hkdf    ├─────►password
      role ──┘      └─────────┘        └────────────┘


```

The tool uses an alphanumeric character set that avoids similarly looking characters.
The generated passwords are 16 characters long, consisting of a set of 32 different characters.
This results in 80 bits of entropy, which is long enough for sufficient security and short enough to make it easy to type into a VNC console.

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
# get seed from enterprise password manager
> SEED=$(op item get --vault network derivepwd-seed --fields password --format=json | jq -j .password)

# derive password for device
> derivepwd --seed=$SEED --role=ipmi router23.prod.example.com
zvhz2zshzbwdv43k
```

