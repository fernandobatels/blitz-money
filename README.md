# Blitz Money - Application for personal financial control

[![Build Status](https://travis-ci.org/fernandobatels/blitz-money.svg?branch=master)](https://travis-ci.org/fernandobatels/blitz-money)
![](https://img.shields.io/github/license/fernandobatels/blitz-money.svg)
![](https://img.shields.io/github/release/fernandobatels/blitz-money.svg)

Inspired on KMyMoney, this application uses on single text file to persist the data, allow OFX files import and can register the future moves on google calendar for notify you.

### Features

- [x] Manage Accounts
- [x] Financial transaction
- [x] OFX import
- [ ] Google Calendar integration
- [ ] Padronization of imported fields(contacts, tags...)
- [ ] Reports
- [ ] Simulations

### How install

- Install the [rust](https://www.rust-lang.org/tools/install) [cargo](https://crates.io/install)
- Clone this repository and run the cargo install:

```bash
git clone https://github.com/fernandobatels/blitz-money && cd blitz-money && cargo install
```

### How to use

- You need a account and a contacts for start creating your transactions
- The interactive mode('-i' option) is better for learning how to use the application
- All data will be saved in ~/.bmoney.bms file. This means that you permissions file system is the responsible for keep the access controll.
- All options for the modules can be visualized typing 'bmoney tags', 'bmoney contacts', 'bmoney accounts' and 'bmoney transactions'
- In all options 'list', like the 'accounts list', you can use the '--use-csv' for get the result in csv instead of a table
