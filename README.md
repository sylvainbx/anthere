# Anthère
Dig up your chat archives

## Setup dev environment
### Install dependencies:
  - docker: `sudo pacman -S docker`
  - libpq: `sudo pacman -S postgresql-libs`
  - diesel: `cargo install diesel_cli --no-default-features --features postgres`
### Create an .env file
  `echo DATABASE_URL=postgres://username:password@localhost/anthere > .env`