<p align="center">
    <a href="https://youtu.be/IOOzoeC-ZRQ?si=118OtD9c8u1Tr7JF">
        <img src="https://raw.githubusercontent.com/pwnwriter/hysp/images/hysp-rounded.png" width="400"></a>
    <br>
    <b><strong>An independent package manager for <code>unix and linux🌷</code></strong></b>
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
* [`Support`](#support)
* [`License`](#license)

<a name="why"></a>
 ## Why?? 🚩

I am a ***CTF player*** who often finds myself without my main laptop in places that lack the tools I need. Some distros don't carry the packages I require and some doesn't keep them updated. That's why I created my own package manager :). It fetches tool binaries easily, ensuring I always have what I need to compete.

<sup><sub>Dont touch my shrug</sub></sup>


<a name="installation"></a>
 ## Installation 📩
    
  <details> <summary><code>🪄 Binary </code></summary>
    &nbsp;

  - *Manual* : You can directly download the [**binary**](https://github.com/pwnwriter/hysp/releases) of your arch and run it.
  - *One liner* : Run this script, requires `jq`,`curl`, `tar` & `wget`
   ```bash
wget -qO- "$(curl -qfsSL "https://api.github.com/repos/pwnwriter/hysp/releases/latest" | jq -r '.assets[].browser_download_url' | grep -Ei "$(uname -m).*$(uname -s).*musl" | grep -v "\.sha")" | tar -xzf - --strip-components=1
./hysp -h
``` 

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
  sudo/doas pacman -Sy hysp
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
 
***Firstly, if you intend to access the binaries installed via `hysp` over the system, you may want to...***

<details> <summary><code>🏵️ Setup path for hysp bin  </code></summary>
 
-  Add the following line to your shellrc. [ `zshrc`, `bashrc` ***etc***. ]

    ```bash
    export PATH="$HOME/.local/share/hysp/bin/:$PATH"
    ```
</details>
 
<details> <summary><code>🐤 Help menu</code></summary>
  &nbsp;
  
  
  ```bash
  hysp |install|uninstall|search| -h # check for help menu
  ```
![screenshot_2023-11-28_13-45-12](https://github.com/pwnwriter/hysp/assets/90331517/ef21f487-961e-4cf9-b87d-1690380dff6a)

</details>

<details> <summary><code>🔻 Installing a pkg </code></summary>
&nbsp;
  
  ```bash
  hysp install -p <pkg> # use --force to overwrite already installed binary 
  ```
  ![screenshot_2023-11-25_22-38-24](https://github.com/pwnwriter/hysp/assets/90331517/f55756b6-b115-4bdf-859f-330f1805c169)

</details>


<details> <summary><code>🧁 Removing a pkg </code></summary>
&nbsp;
  
  ```bash
  hysp remove -p <pkg> 
  ```

![screenshot_2023-11-27_18-56-49](https://github.com/pwnwriter/hysp/assets/90331517/e468c329-eb08-4b08-8c06-6a0e56756ee5)

</details>

<details> <summary><code>🔭 Search for available pkgs </code></summary>
&nbsp;
  
  ```bash
  hysp search -p <pkg> # use --silent to strip down the console i/o
  ```

![screenshot_2023-11-26_14-24-57](https://github.com/pwnwriter/hysp/assets/90331517/19a837c4-45cf-4043-86ac-b83cf780c487)

</details>

<details> <summary><code>⚕️ Checking configuration health </code></summary>
&nbsp;
  
  ```bash
 hysp health
  ```

![screenshot_2023-11-28_13-51-37](https://github.com/pwnwriter/hysp/assets/90331517/27d78886-2e3f-4396-8b73-a9a26facad41)

</details>

<a name="repo"></a>
 ## Hosting custom repo 💾

- Hysp provies the following configuration, which can be overwritten by defining a `config file`.
  Default config

  ```toml
    [source]
    remote = "https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/"
    aarch = "Architecture"

    [local]
    home="/home/user/.local/share/hysp"
    bin="/home/user/.local/share/hysp/bin/" 
    data="/home/user/.local/share/hysp/data/Architecture/" 
  ```
- Explanation 

|  Name       | Description                        | Default                                            |
|-------------|------------------------------------|----------------------------------------------------|
|  `remote`   | Package repository                 | [***`metis-os/hysp-pkgs`***](https://github.com/metis-os/hysp-pkgs) |
|  `home`     | Home for `hysp`                    | ***`hysp`***                               |
|  `bin`      | Directory to save the binaries     | ***`~/.local/share/hysp/bin`***            |
|  `data`     | Directory to save pkg data         | ***`~/.local/share/hysp/data`***           |
|  `aarch`    | Your system Architecture           | Only supported ***`X86_64,aarch64`***      |

<details> <summary><code>🎄 Tree view of the repo </code></summary>
&nbsp;

  ```bash
.
├── available.toml # Storing available pkgs info (Optional)
├── data
│   └── x86_64 # Your cpu Architecture (aarch64 and x86_64) supported for now
│       └── foo.toml # where the package data are stored (needed)
```

</details>




<details> <summary><code>📂 Sample pkg </code></summary>
&nbsp;

  ```bash
[bin]
name = "$BIN" # Name of the pkg to be installed as

[package]
architecture = "x86_64" # Your aarchitecture 
name = "$BIN" # Your package name
description = "$DESCRIPTION" # Description
author = "$AUTHOR" # Author 
repo = "$REPO_URL" 
stars = "${STARS}"
version = "$PKG_VERSION"
updated = "$PKG_RELEASED"
size = "$SIZE"
sha = "$SHA" 
source = "$SOURCE_URL" # Source of the binary wherever it's hosted
language = "$LANGUAGE"
license = "$LICENSE"

[package.conditions]
conflicts  = ["$BIN"] # Conflictions 
requires = [] # Dependencies 

[package.metadata]
keywords = $TOPICS
categories = ["Utilities"]
  ```

</details>

<a name="pkgs"></a>
 ## Packages whuat?? 📦
There is a list of packages available in [*`metis-os/hysp-pkgs`*](https://github.com/metis-os/hysp-pkgs) . You can confidently utilize the default configuration without any hesitation. However, if you prefer to host your own packages, you have the option to do so by creating your own custom configuration file under ***`~/.config/hysp/config.toml`***. See [`#repo`](https://github.com/pwnwriter/hysp#repo) 


<a name="support"></a>
 ## Support 💌

 I am a student currently attending university. I like working for *Open Source* in my free time. If you find my tool or work beneficial, please consider supporting me via [*KO-FI*](https://ko-fi.com/pwnwriter) or [*ESEWA*](https://metislinux.org/docs/donate)* (***Nepal only***), Or by leaving a star ⭐ ; I'll appreciate your action :)

<a name="license"></a>
 ## License ㊙️

 Everything is license under the [`MIT`](https://raw.githubusercontent.com/pwnwriter/hysp/main/LICENSE) except for the packages... 
 They hold their own livess :oOO
 
<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023<a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz </a> ☘️</p> 




