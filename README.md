# iivanovw7-dev

### Main crates

-   [axum](https://docs.rs/axum/latest/axum)
-   [askama](https://docs.rs/askama/latest/askama)
-   [tokio](https://docs.rs/tokio/latest/tokio)

### Requirements

-   Cargo 1.72.0
-   Rustc 1.72.0 (5680fa18f 2023-08-23) (Arch Linux rust 1:1.72.0-1)

### Installation

`.env` file example

```bash
SERVER="127.0.0.1:9000"
```

-   Install rust

```bash
    pacman -S rustup
    rustup default stable
```

-   Install pnpm

```bash
  npm install --global pnpm
```

-   Setup node version manager environment

```bash
   nvm use # or nvm install
```

### Scripts

-   Development server

```bash
    pnpm run dev
    pnpm run dev:css
```
