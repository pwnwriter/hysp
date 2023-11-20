### Pros
    
    - Uses fastest way(blake3) to check hash of a binary
    - Easy packaging with toml  
    - Supports *nix and windows as well


### Cons
    
    - Installation of a single binary is Supported (at least for now)
    - Needs raw binary to be hosted



### Variables

    - Read pkg repo from env 
            
        ```bash
        $ SEREN_REPO_URL
        ```

    - Read pkg binary store location from env

        ```bash
        $ SEREN_BIN_DIR
        ```
    - Read data directory

        ```bash
        $ SEREN_DATA_DIR
        ```
