# ZWEB cli subcommands

## Deploy

Command name: `deploy`

Use: Deploy a new ZWEB Builder Docker instance with the given name `zweb_builder`.

Options:

- `-S, --self`: Self-hosted installation

- `-C, --cloud`: ZWEB Cloud installation

- `-V, --builder-version <X.Y.Z>`: Set the version of ZWEB Builder. The default value is `latest`.

- `-p, --port <PORT>`: Set the port of ZWEB Builder. The default value is `80`.

- `-m, --mount <PATH>`: The mount path for the ZWEB Builder. The default value is `/var/lib/zweb`.

- `-h, --help`: Prints help information

## Stop

Command name: `stop`

Use: Stop one or more ZWEB Builder.

Options:

- `-S, --self`: Stop Self-hosted ZWEB Builder

- `-C, --cloud`: Stop ZWEB Builder on ZWEB Cloud

- `-h, --help`: Prints help information

## Restart

Command name: `restart`

Use: Restart one or more ZWEB Builder.

Options:

- `-S, --self`: Restart Self-hosted ZWEB Builder

- `-C, --cloud`: Restart ZWEB Builder on ZWEB Cloud

- `-h, --help`: Prints help information

## Remove

Command name: `remove`

Use: Remove one or more ZWEB Builder.

Options:

- `-S, --self`: Remove Self-hosted ZWEB Builder

- `-C, --cloud`: Remove ZWEB Builder on ZWEB Cloud

- `-f, --force`: Force the removal of a ZWEB Builder Docker instance (uses SIGKILL)

- `-d, --data`: Remove the persistent data of ZWEB Builder

- `-h, --help`: Prints help information

## Update

Command name: `update`

Use: Update ZWEB Builder to latest version.

Options:

- `-S, --self`: Update Self-hosted ZWEB Builder

- `-C, --cloud`: Update ZWEB Builder on ZWEB Cloud

- `-h, --help`: Prints help information

## List

Command name: `list`

Use: List ZWEB Builder.

Options:

- `-A, --all`: All ZWEB Builder

- `-S, --self`: List Self-hosted ZWEB Builder

- `-C, --cloud`: List ZWEB Builder on ZWEB Cloud

- `-h, --help`: Prints help information

## Doctor

Command name: `doctor`

Use: Check the pre-requisites of self-host.

Options:

- `-h, --help`: Prints help information

## Help

Command name: `help`

User: Print help information.