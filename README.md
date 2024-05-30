# Livewire

- [Description](#description)
- [Getting Started](#getting-started)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [License](#license)

## Description

Cross platform WiFi management tool

## Getting Started


### Installing

```sh
cargo add livewire
```

### Executing program

```rust
use livewire::{WifiHotspotConfig, WifiHotspotConfigBuilder};

fn main() {

let available_networks = WifiHotspotConfig::scan();

    println!("Hello, world!, {:#?}", available_networks);
}
```


## Documentation

See the crate documentation on [doc.rs](https://docs.rs/leptos-remix-icon/latest/livewire/)

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details

