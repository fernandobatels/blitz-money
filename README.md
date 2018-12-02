# Blitz Money - Application for personal financial control

Inspired on KMyMoney, this application uses on single text file to persist the data, allow OFX files import and can register the future moves on google calendar for notify you.

### Features

- [x] Manage Accounts
- [x] Financial transaction
- [ ] OFX import
- [ ] Google Calendar integration
- [ ] Padronization of imported fields(contacts, tags...)
- [ ] Reports
- [ ] Simulations

### How to use

- In all options 'list', like the 'accounts list', you can use the '--use-csv' for get the result in csv instead of a table
- The interactive mode('-i' option) is better for learning how to use the application
- All data will be saved in ~/.bmoney.bms file. This means that you permissions file system is the responsible for keep the access controll.

#### Accounts

New accounts:

```shell
bmoney accounts add [name] [bank] [opening balance date] [opening balance] [currency]
# Or with interactive mode:
bmoney accounts add -i
```

Editing account:

```shell
bmoney accounts update [id] [name|bank|obd|ob|curency] [value]
#Or with interactive mode:
bmoney accounts update -i
```

Your accounts:

```shell
bmoney accounts list
```

Current status of your accounts:

```shell
bmoney accounts status
```

Removing account:

```shell
bmoney accounts rm [id]
```

#### Contacts

New contacts:

```shell
bmoney contacts add [name] [city]
#Or with interactive mode:
bmoney contacts add -i
```

Editing contact:

```shell
bmoney contacts update [id] [name|city_location] [value]
#Or with interactive mode:
bmoney contacts update -i
```

Your contacts:

```shell
bmoney contacts rm list
```

Removing contact:

```shell
bmoney contacts rm [id]
```

#### Tags

New tags:

```shell
bmoney tags add [name]
#Or with interactive mode:
bmoney tags add -i
```

Editing tag:

```shell
bmoney tags update [id] [name] [value]
#Or with interactive mode:
bmoney tags update -i
```

Your tags:

```shell
```

Removing tag:

```shell
bmoney tags rm [id]
```

#### Transactions

- For make transfer between accounts you only need put id of destination account instead of the id of contact

New transactions:

```shell
bmoney transactions add [description] [value] [account id] [contact id] [deadline] [paid in] [tags] [observations]
#Or with interactive mode:
bmoney transactions add -i
```

Editing transaction:

```shell
bmoney transactions update [id] [description|value|account|contact|deadline|paid|tags|observations] [value]
#Or with interactive mode:
bmoney transactions update -i
```

Your transactions:

```shell
bmoney transactions list [account id] [from] [to]
```

Removing transaction:

```shell
bmoney transactions rm [id]
```
