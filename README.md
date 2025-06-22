# iivanovw7-dev

## Main crates

-   [axum](https://docs.rs/axum/latest/axum)
-   [askama](https://docs.rs/askama/latest/askama)
-   [tera](https://keats.github.io/tera/docs/#introduction)

### Requirements

-   Cargo 1.85.0
-   rustc 1.85.0

### Installation

`.env` file example

```bash
SERVER="0.0.0.0:9000"
```

-   Install `rust`

```bash
pacman -S rustup
rustup default stable
```

-   Install `rustfmt` and `rust-analyzer`

```bash
rustup component add rust-analyzer
rustup component add rustfmt
```

-   Install `pnpm`

```bash
npm install --global pnpm
```

-   Setup node version manager environment

```bash
nvm use # or nvm install
```

-   `Tera` formatting

```bash
sudo pacman -S python-pipx
pipx install djlint
```

### Scripts

-   Development server

```bash
pnpm run dev
pnpm run dev:css
```

### Dockerfile

-   Install docker

```bash
sudo pacman -Syu
sudo pacman -S docker
sudo systemctl start docker.service
sudo systemctl enable docker.service
sudo usermod -aG docker $USER

sudo docker version

sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

docker-compose --version
# docker-compose version 1.29.2, build 5becea4c

```

-   Build and run container

```bash
docker build -t container-name .
docker run -d -p 3000:8080 --name container-name container-name

# clear cache
docker system prune -a
docker image prune

# Update docker image after changes
chmod +x recompose.sh
./recompose
```
