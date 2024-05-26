_**This project has been archived and is no longer maintained.**_

WHY? 

_I recently started university and am currently busy with my personal work.
Additionally, I've begun using `nix`, and this project was initially a way for me to learn about package managers and their internals. Therefore, I am archiving this project and do not recommend anyone use it. However, feel free to borrow any parts of it for your own projects if you find them inspiring._


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
        <img src="https://img.shields.io/github/actions/workflow/status/pwnwriter/hysp/build-app.yml?style=flat&labelColor=eba0ac&color=74c7ec&label=check-hysp&logo=GitHub%20Actions&logoColor=white">
    </a>
  <a href="https://github.com/pwnwriter/hysp/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-white.svg" alt="MIT LICENSE"></a>
  <br>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/palette/macchiato.png" width="500" />
</p>

## Table of contents 📔

* [`Why`](#why)
* [`Features`](#features)
* [`Installation`](#installation)
* [`Hysp usages`](#usages)
* [`Hosting custom repo`](#repo)
* [`Packages`](#pkgs)
* [`Support`](#support)
* [`License`](#license)

<a name="why"></a>
 ## Why?? 🚩

I am a **CTF player** [***(Capture the flag)*** ](https://en.wikipedia.org/wiki/Capture_the_flag_(cybersecurity)) who often finds myself without my main laptop in places that lack the tools I need. Some distros don't carry the packages I require and some doesn't keep them updated. That's why I created my own package manager :). It fetches tool binaries easily, ensuring I always have what I need to compete.

<sup><sub>Dont touch my shrug</sub></sup>

<a name="features"></a>
 ## Features ⚾
1. **Versatile** : Hysp operates on (*nix) systems and various architectures (x86, aarch64_arm), providing flexibility for your needs.
2. **Simple** : No prerequisites is needed for `Hysp` or `hysp-pkgs`. Install Hysp as a single binary, no need for Go, Rust, or any other dependencies. Saves space, storage, and time.
3. **Customizable** : Pkg-Source can be self-hosted by anyone, allowing hysp to use your ***very own*** instead of the default source and local configuration as well. See [#Self hosting](https://github.com/pwnwriter/hysp#repo)
4. **Statically compiled** : The default source [`metis-os/hysp-pkgs`](https://github.com/metis-os/hysp-pkgs)) has all statically compiled binaries, _only statically linked binaries that will run anywhere_. You can always host dynamic or whatever you want.
5. **No Special Privileges** : [Hysp](https://github.com/pwnwriter/hysp) requires no special perms or privileges. It can run completely in userspace with all of its features.
6. **Everything is open source** : Hysp uses github actions to build and ship the packages. On top of that it uses `sha` for varifying the binary.
  
<details> <summary><code>7. Security Considerations</code></summary>
  &nbsp;

It is never a good idea to install random binaries from random sources. 

Check these `HackerNews Discussions`
> - [A cautionary tale from the decline of SourceForge](https://news.ycombinator.com/item?id=31110206)
> - [Downloading PuTTY Safely Is Nearly Impossible (2014)](https://news.ycombinator.com/item?id=9577861)


> ```bash
> !# PKG Metadata
> # Everything is automated via Github Actions & Scripts
> Repo --> https://github.com/metis-os/hysp-pkgs
> WorkFlows --> https://github.com/metis-os/hysp-pkgs/tree/main/.github/workflows
> Scripts --> https://github.com/metis-os/hysp-pkgs/tree/main/.github/scripts
> 
> !# Upstream Source
> # Everything is automated via Github Actions & Build Scripts
> Repo --> https://github.com/Azathothas/Toolpacks
> WorkFlows --> https://github.com/Azathothas/Toolpacks/tree/main/.github/workflows
> Build Scripts --> https://github.com/Azathothas/Toolpacks/tree/main/.github/scripts
> ```
  </details>



<a name="installation"></a>
 ## Installation 📩

###### 🐤 From source

  ```bash
  git clone --depth=1 https://github.com/pwnwriter/hysp --branch=main
  cd hysp
  cargo build --release 
  ```
  Then go to `release` dir and `./hysp` or move the `binary` to your any `$PATH` for instant access from anywhere.

    
  <details> <summary><code>🪄 Binary </code></summary>
    &nbsp;

  - *Manual* : You can directly download the [**binary**](https://github.com/pwnwriter/hysp/releases) of your arch and run it.
  - *One liner* : Run this script, requires `jq`,`curl`, `tar` & `wget`
   ```bash
wget -qO- "$(curl -qfsSL "https://api.github.com/repos/pwnwriter/hysp/releases/latest" | jq -r '.assets[].browser_download_url' | grep -Ei "$(uname -m).*$(uname -s).*musl" | grep -v "\.sha")" | tar -xzf - --strip-components=1
./hysp -h
```
</details>


<details> <summary><code>💮 using Cargo </code></summary>
&nbsp;

- Using [crates.io](https://crates.io/crates/hysp)
  ```bash
  cargo install hysp
  ```
- Using [binstall](https://github.com/cargo-bins/cargo-binstall)
  ```bash
  cargo binstall hysp
  ```

    > **NOTE:**
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

 ##### Setup path for hysp bin
 
-  Add the following line to your shellrc. [ `zshrc`, `bashrc` ***etc***. ]

    ```bash
    export PATH="$HOME/.local/share/hysp/bin/:$PATH" # While using default config, else use your own path to bin
    ```
 
<details> <summary><code>Help menu</code></summary>
  &nbsp;
  
  
  ```bash
  hysp |install|uninstall|search| -h # check for help menu
  ```

![screenshot_2023-12-13_19-51-00](https://github.com/pwnwriter/hysp/assets/90331517/01f15c0b-6b73-4e7c-ae74-5d010bef10f9)

</details>

<details> <summary><code>Installing packages </code></summary>
&nbsp;
  
  ```bash
  hysp install -p <foo,bar,buzz> # use --force to overwrite already installed binary, --quiet to supress console io
  ```
![screenshot_2023-12-13_19-55-36](https://github.com/pwnwriter/hysp/assets/90331517/79ce202a-23a4-4086-bd47-66edd0718345)

</details>


<details> <summary><code>Removing packages </code></summary>
&nbsp;
  
  ```bash
  hysp remove -p <foo,bar,buzz> 
  ```

![screenshot_2023-12-13_19-57-26](https://github.com/pwnwriter/hysp/assets/90331517/84841cf4-0693-4cbf-a2cc-b46869596b94)

</details>

<details> <summary><code>Search for available pkgs </code></summary>
&nbsp;
  
  ```bash
  hysp search -p <pkg> 
  ```

- Raw mode (default)

![screenshot_2023-12-13_19-58-22](https://github.com/pwnwriter/hysp/assets/90331517/c72bfd75-b246-4b9d-82b9-0c11e399c947)

- Database mode

![screenshot_2023-12-13_19-59-55](https://github.com/pwnwriter/hysp/assets/90331517/66e1a7f2-9815-41c2-8da5-8e0144789d38)

- Fuzzy mode

![screenshot_2023-12-13_20-00-34](https://github.com/pwnwriter/hysp/assets/90331517/0404c9d9-2049-459e-b09d-253dfbe30a4d)

</details>

<details> <summary><code>Checking configuration health </code></summary>
&nbsp;
  
  ```bash
 hysp health
  ```

![screenshot_2023-12-13_20-01-34](https://github.com/pwnwriter/hysp/assets/90331517/2375116f-bda1-4dd9-96f9-48f04fa8bc47)

</details>

##### Numerous other options exist. Consider installing Hysp and checking it out, wouldn't you?

<a name="repo"></a>
 ## Hosting custom repo 💾

- Hysp provies the following configuration, which can be overwritten by defining a `config file` in `~/.config/hysp/config.toml`

  ```toml
  [source]
  remote = "https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/x86_64"
  metadata ="https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/metadata.toml"
  aarch = "Architecture"

  [local]   
  home="/home/user/.local/share/hysp"
  bin="/home/user/.local/share/hysp/bin/" 
  data="/home/user/.local/share/hysp/data/" 

  ```
- Explanation 

|  Name       | Description                        | Default                                            |
|-------------|------------------------------------|----------------------------------------------------|
|  `remote`   | Package repository                 | [***`metis-os/hysp-pkgs`***](https://github.com/metis-os/hysp-pkgs) |
|  `metadata` | Available pkg info                 | [***`metis-os/hysp-pkgs/metadata.toml`***](https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/metadata.toml)|
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
│  └── foo.toml # where the package data are stored (needed)
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




