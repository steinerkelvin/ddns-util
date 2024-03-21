# k-ddns

Auto detects IP and update DDNS records on [Dynv6](https://dynv6.com/).
Can update IPv6 only.

## Usage

```
Usage: k-ddns [OPTIONS]

Options:
  -4, --detect-ipv4 <DETECT_IPV4>  [default: auto] [possible values: auto, nope]
  -6, --detect-ipv6 <DETECT_IPV6>  [default: auto] [possible values: auto, nope]
  -h, --help                       Print help
```

### Example

```sh
DYNV6_TOKEN="<your_dynv6_token>" \
DOMAIN=<you_domain>.dynv6.net \
k-ddns -4 auto -6 auto
```

## Features

- Can update IPv6 only
- Auto detects IPv4 and IPv6

### TODO

- [ ] Pass IP values manually.
- [ ] More IP detection providers.

# Systemd service

We provide [a NixOS module](https://github.com/steinerkelvin/k-ddns/blob/master/nix/modules/k-ddns.nix)
at `thisFlake.nixosModules.k-ddns` to run this tool periodically with systemd timers.

Someone should probably add example systemd definition files too tho.
