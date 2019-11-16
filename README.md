# ShowMan

_**Show Man**ager_ software is a web application for organizing theater plays, handling various aspects such as scripting, direction and scenography.

The initial goal of this software is to organize the annual Christmas show in the Mathematics Department of my university; however, once completed, can be easily extended to generic theater plays. 

## Installation and configuration

### End users

Being this a web application, there is no software to install for end users.
However, they need to register to the service.

### Installation

This installation guide is intended for the owner of the server.

The mandatory software is:
- Cargo ([Rust](https://www.rust-lang.org/) packet manager)
- [Diesel](http://diesel.rs) (ORM for Rust applications)
- MySql (or another DBMS, with some further configuration)
- OpenSSL

At the moment this crate is not hosted on [crates.io](https://crates.io).
However, the software can be installed by downloading the repository and using the command `cargo install`.

After the [configuration](#configuration), the server can be started simply typing `showman` on the command line.

### Configuration

After configuring `diesel`, you should generate two files: `.env` and `pass.key`.
Moreover, you need an SSL key/cert pair stored in files `key.pem` and `cert.pem`.

#### `.env`

The `.env` file is created manually and contains additional environment variables.
At the moment, the only needed variable is the database location, thus the file should be similar to the following.
```dotenv
BIND=<address>:<port>
BIND_SSL=<address>:<port>
DATABASE_URL=mysql://<user>:<password>@<location>/<db_name>
```
The `DATABASE_URL` parameters are the following:
- `user` is the database user that can perform the operations;
- `password` is the password of the above user;
- `location` is the domain or ip address of the database server;
- `db_name` is the name of the database (`showman` is highly recommended).

##### Notes

The `.env` is not necessary if a `DATABASE_URL` environment variable exists with the same value.

#### `pass.key`

The `pass.key` file contains the JWT token signing and verification key.
To generate it from random bytes, simply type in a shell the following line.
``` 
openssl rand -out pass.key 64
```

## To-Do

There is much work to be done at the moment.
The following list contains a brief summary of the most urgent tasks.

- [ ] Refactor and simplify the authentication/GUI part.
- [ ] Complete the CLI user management part.
- [ ] Add shows management.

The list may be updated in the future as the project goes on.

## License

This software is licensed under [MIT](LICENSE) license.