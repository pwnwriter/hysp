<p align="center">
    <a href="https://youtu.be/IOOzoeC-ZRQ?si=118OtD9c8u1Tr7JF">
        <img src="https://raw.githubusercontent.com/pwnwriter/hysp/images/hysp-rounded.png" width="400"></a>
    <br>
    <b><strong>An independent package manager for the <code>unix and linux🌷</code></strong></b>
    <br>
    <br>
    <a href="https://github.com/pwnwriter/hysp/releases">
        <img src="https://img.shields.io/github/v/release/pwnwriter/hysp?style=flat&labelColor=f38ba8&color=585b70&logo=GitHub&logoColor=white">
    </a>
    <a href="https://crates.io/crates/hysp/">
        <img src="https://img.shields.io/crates/v/hysp?style=flat&labelColor=b4befe&color=eba0ac&logo=Rust&logoColor=white">
    </a>
    <a href="https://github.com/pwnwriter/hysp/actions?query=workflow%3A%22Continuous+Deployment%22">
        <img src="https://img.shields.io/github/actions/workflow/status/pwnwriter/hysp/test-app.yml?style=flat&labelColor=eba0ac&color=74c7ec&label=Test-app&logo=GitHub%20Actions&logoColor=white">
    </a>
  <a href="https://github.com/pwnwriter/hysp/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-white.svg" alt="MIT LICENSE"></a>
  <br>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/palette/macchiato.png" width="500" />
</p>

## Table of contents 📔

* [`Why`](#why)
* [`Installation`](#installation)
* [`Hysp usages`](#usages)
* [`Hosting custom repo`](#repo)
* [`Packages`](#pkgs)
* [`License`](#license)

<a name="why"></a>
 ## Why?? 🚩

I am a ***CTF player*** who often finds myself without my main laptop in places that lack the tools I need. Some distros don't carry the packages I require and some doesn't keep them updated. That's why I created my own package manager :). It fetches tool binaries easily, ensuring I always have what I need to compete.

<sup><sub>dont touch my shrug</sub></sup>


<a name="installation"></a>
 ## Installation 📩
    
  <details> <summary><code>🪄 Binary </code></summary>
    &nbsp;

  - You can directly download the [**binary**](https://github.com/pwnwriter/hysp/releases) of your arch and run it.
  
  </details>
  <details> <summary><code>🌼 Source </code></summary>
  &nbsp;
 
  ```bash
  git clone --depth=1 https://github.com/pwnwriter/hysp --branch=main
  cd hysp
  cargo build --release 
  ```
  Then go to `release` dir and `./hysp` or move the `binary` to your any `$PATH` for instant access from anywhere.
</details>

<details> <summary><code>🎠 Cargo </code></summary>

- Using [crates.io](https://crates.io/crates/hysp)
  ```bash
  cargo install hysp
  ```
- Using [binstall](https://github.com/cargo-bins/cargo-binstall)
  ```bash
  cargo binstall hysp
  ```

  > **Note** ⚠️
  > This requires a working setup of rust/cargo & binstall.
</details>

<details> <summary><code>🚩 METIS Linux </code></summary>
&nbsp;
  
  ```bash
  sudo/doas pacman -Syyy hysp
  ```

</details>

<details> <summary><code>💢 Arch user repository </code></summary>
&nbsp;
  
  ```bash
  paru/yay -S hysp-git
  ```

</details>


<a name="usages"></a>
 ## Hysp usages 🎠
<details> <summary><code> Help menu🐤 </code></summary>
  &nbsp;
  
  
  ```bash
  hysp |install|uninstall|search| -h # check for help menu
  ```
  ![screenshot_2023-11-21_23-01-39](https://github.com/pwnwriter/hysp/assets/90331517/b10a4832-a8cc-4017-98d2-019c048a0f95)
</details>

<details> <summary><code>🔻 Installing a pkg </code></summary>
&nbsp;
  
  ```bash
  hysp install -p <pkg> # Use --silent to supress console output
  ```
![screenshot_2023-11-21_23-02-55](https://github.com/pwnwriter/hysp/assets/90331517/ef4577b3-de8b-4992-b24c-8552eb20ed05)

</details>


<details> <summary><code>🧁 Removing a pkg </code></summary>
&nbsp;
  
  ```bash
  hysp remove -p <pkg> # Use --silent to supress console output
  ```
</details>


<a name="repo"></a>
 ## Hosting custom repo 💾


<details> <summary><code> Setup path for hysp bin🎡  </code></summary>
    
-  Add hysp binaries to `$PATH` for ease access over the system

    ```bash
    export PATH="$PATH:${$(find ~/.local/share/hysp/bin -type d -printf %p:)%%:}"
    ```
</details>

- Hysp provies the following envirovnment variables

| Variable        | Description                        | Default                                            |
|-----------------|------------------------------------|----------------------------------------------------|
| `HYSP_REPO_URL` | Package repository                 | [***`metis-os/hysp-pkgs`***](https://github.com/metis-os/hysp-pkgs) |
| `HYSP_BIN_DIR`  | Directory to save the binaries     | ***`~/.local/share/hysp/bin`***            |
| `HYSP_HOME_DIR` | Home for `hysp`                    | ***`hysp`***                               |
| `HYSP_DATA_DIR` | Directory to save pkg data         | ***`~/.local/share/hysp/data`***           |

<details> <summary><code>🎄 Tree view of the repo </code></summary>
&nbsp;

  ```bash

├── available.toml ## all pkgs info are stored here
├── data
│   ├── foo.toml ## specific pkg information are stored here 
├── LICENSE
└── pkgs
    ├── foo ## pkgs binary are stored here
```

</details>




<details> <summary><code>📂 Sample pkg </code></summary>
&nbsp;

  ```bash
[package]
name = "foo"
version = "x.y.z"
description = "A sample package for demonstration purposes"
license = "bar"
size = "x.yM"

[maintainer]
name = "foo " #Maintainer infos
email = "foo@bar.com"

[source]
url = "https://github.com/metis-os/hysp-pkgs/raw/main/pkgs/foo" # Binary url

[bin]
name = "foo"  # Name of the binary executable

[package.conditions]
conflicts  = [ "foo"  ] # Example conflict entry
dependencies = [ "foo" ]  # Example dependency entry

[package.metadata]
hash = "57f8c02b16eefe47cc099336f43c3f5e491c34bd446c9b32f33c9da29adebd5d" # Optional b3sum (Yet to implement hash checking)
keywords = ["sample", "demonstration", "rust"] # Optional
categories = ["Utilities", "Development"] # Needed

  ```

</details>

> Note: 🗒️ Once you create a package you must fill the `available.toml`. [`hysp`](/) uses that file to track the packages.

<a name="pkgs"></a>
 ## Packages whuat?? 📦

Yeah, I agree that currently, there are very few packages available, easily countable by hand. Therefore, contributions are more than welcome.
Either to the [`packages`](https://github.com/metis-os/hysp-pkgs) for your package or here the [`core app`](/) itself. If you see anything that can be improved... Just create an [`issue`](https://github.com/pwnwriter/hysp/issues) // [`pr`](https://github.com/pwnwriter/hysp/pulls) :V

<a name="license"></a>
 ## License ㊙️

 Everything is license under the [`MIT`](https://raw.githubusercontent.com/pwnwriter/hysp/main/LICENSE) except for the packages... 
 They hold their own livess :oOO

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023<a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz </a> ☘️</p> 




