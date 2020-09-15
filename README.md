# ShowMan

_**Show Man**ager_ software is a web application for organizing theater plays, handling various aspects such as scripting, direction and scenography.

The initial goal of this software is to organize the annual Christmas show in the Mathematics Department of my university; however, once completed, can be easily extended to generic theater plays. 

At the moment, the project is under complete rework.

## Installation and configuration

1. Install MySql server/community/whatever.
2. Install OpenSSL.
3. Install Diesel (`cargo install diesel --no-default-features features mysql`).
4. `git clone` repository.
5. Create `.env` file as follows:
    - `DATABASE_URL=mysql://server:server_password@127.0.0.1/showman` (replace `server` and `server_password` with username and password);
    - `MIGRATION_DIRECTORY=assets/migration`;
    - `API_HOSTNAME=api.localhost`;
    - `AUTH_HOSTNAME=auth.localhost`;
    - `HOSTNAME=localhost`.
6. Run `diesel setup`.

**TODO:** (partially) automate process.

## To-Do

- [x] API (base)
- [ ] GUI (base)
- [ ] AUTH
- [ ] API (full)
- [ ] GUI (full)
- [ ] Readme
- [ ] Setup

## License

This software is licensed under [MIT](LICENSE) license.