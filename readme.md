# rmig 0.0.1 by [tux](https://github.com/realtux)

rmig is a re-implementation (and re-imagination) of an [old c project](https://github.com/realtux/bmig) which handled
mysql migrations.
it shores up any lingering problems from the previous project and builds upon it with newer features.
this project, like the previous one, aims to be a generic database migrations manager which will support several platforms, but mysql at first.

## installation

in general, see releases to download the binary for your platform. if you want to compile on your own,
make sure you're using at least `1.41.0` of the rust toolchain.

```
git clone https://github.com/realtux/rmig
cd rmig
cargo build --release
```

then do something with `target/release/rmig`.

## rmig commands

### initialize rmig
```
rmig init
```
this will ask you a few questions and create a `config.json` file in the current directory as long
as one doesn't already exist. if one already exists, rmig will say so and the program will exit.

***options***

`-f` force init and overwrite `config.json` if it exists

***generated json***
```json
{
  "host": "localhost",
  "port": 3306,
  "user": "root",
  "pass": "root",
  "db": "mydb",
  "platform": "mysql"
}
```
your data will vary according to what you've put in either manually or via `rmig init`.

#

### create a new migration
```
rmig create [name]
```
this will create a new file in the format of `[timestamp]-[name].sql` in the migrations folder.
the name can be a single word, multiple words, or whatever you'd like. each space is replaced with a dash.
for instance, `rmig create my new migration` will result in a migration named `[timestamp]-my-new-migration.sql`.
it will contain an `up:` and `down:` label. there must be a newline after each label and a newline
after each command you write. if you don't have an `up` or a `down`, you can leave the label out of
the migration, or leave it blank; either way is fine.

#

### check the status of each migration
```
rmig status
```
this will check each migration in the migrations folder
and compare that against what is in the `rmig` table that `rmig` will create in the
beginning. migrations that are present in the table will be marked as `up` and
migrations that are not will be marked as `dn`. a count of pending migrations will be shown.

#

### apply all pending migrations
```
rmig migrate
```
this will run each migration marked as `dn` from `rmig status` and execute the contents from
the `up:` label located in that migration. if multiple migrations are marked as `dn`,
they will be ran sequentially starting with the oldest.

#

### rolling back migrations
```
rmig rollback
```
this will rollback each migration marked as `up` from `rmig status` and execute
the contents from the `down:` label located in that migration. by default, only the most
recently migration will be rolled back.

#

### license

rmig is available under the MIT License
