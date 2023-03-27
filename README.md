# Help Desk Server

> The backend service of "Help Desk" project.

# 🚧 WARNING: UNDER CONSTRUCTION 🚧

This project is work in progress and subject to frequent changes.

# Development

To set up local environment, please follow the following instructions.

## Prerequisites

- mysql client (optional)
- `diesel_cli` (you can install by running `cargo install diesel_cli`)
- nats client (optional)

## start up data storage processes.

Start data storage processes by `docker-compose up`.

- [TiDB](https://github.com/pingcap/tidb)
- [dragonflydb](https://github.com/dragonflydb/dragonfly)
- [NATS](https://github.com/nats-io/nats-server)
- [Meilisearch](https://github.com/meilisearch/meilisearch)

**notes**

- dragonflydb version require `v0.10.0` or later. (we use `SMISMEMBER` command)

```bash
% cd help-desk-server  # project root directory
% docker-compose-up
```

## create database and user if not exists

Connect tidb with mysql client installed on your development machine.

```bash
% mysql -h 127.0.0.1 -P 4000 -u root
```

Inside tidb console, execute the following queries.

```bash
# confirm tidb version
mysql> SELECT tidb_version();

mysql> CREATE DATABASE IF NOT EXISTS help_desk;

# create user and grant privileges.
mysql> CREATE USER 'help_desk'@'%' IDENTIFIED BY 'password';
mysql> SELECT Host, User FROM mysql.user;
+------+-----------+
| Host | User      |
+------+-----------+
| %    | help_desk |
| %    | root      |
+------+-----------+

mysql> GRANT ALL PRIVILEGES ON help_desk.* TO 'help_desk'@'%' WITH GRANT OPTION;
mysql> SHOW GRANTS FOR 'help_desk'@'%';
+--------------------------------------------------------------------------+
| Grants for help_desk@%                                                   |
+--------------------------------------------------------------------------+
| GRANT USAGE ON *.* TO 'help_desk'@'%'                                    |
| GRANT ALL PRIVILEGES ON help_desk.* TO 'help_desk'@'%' WITH GRANT OPTION |
+--------------------------------------------------------------------------+
```

## run migrations

Run migration scripts by the following command.

See [Diesel](https://diesel.rs/) for more information.

```bash
% export DATABASE_URL=mysql://help_desk:password@127.0.0.1:4000/help_desk?charset=utf8mb4
% diesel migration run
```

## initialize meilisearch default API key

```bash
% curl \
  -X POST 'http://localhost:7700/keys' \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer MASTER_KEY' \
  --data-binary '{
    "description": "Default Search API Key",
    "actions": ["*"],
    "indexes": ["*"],
    "expiresAt": null
  }'
{
  "name": null,
  "description": "Default Search API Key",
  "key": "01d5e2eaaaee7a36104ff786f5621b3f21a41ddd628ca12f6fc0b157cfc109ff",
  "uid": "45a7ed19-1926-44af-910b-cb13c3b0c97c",
  "actions": [
    "search"
  ],
  "indexes": [
    "*"
  ],
  "expiresAt": null,
  "createdAt": "2022-12-30T14:44:19.420945887Z",
  "updatedAt": "2022-12-30T14:44:19.420945887Z"
}
# then you can use `01d5e2eaaaee7a36104ff786f5621b3f21a41ddd628ca12f6fc0b157cfc109ff` as api key.
% export MEILISEARCH_API_KEY=01d5e2eaaaee7a36104ff786f5621b3f21a41ddd628ca12f6fc0b157cfc109ff
```

## initialize NATS stream/consumers

```bash
% nats str ls
No Streams defined
# if no stream exists, add new stream by the following command.
% nats str add SEARCH \
  --subjects "search" \
  --ack \
  --max-msgs=-1 \
  --max-bytes=-1 \
  --max-age=1y \
  --storage=file \
  --retention=limits \
  --max-msg-size=-1 \
  --max-msgs-per-subject=-1 \
  --discard=old \
  --dupe-window="2m0s" \
  --replicas=1

% nats con ls SEARCH
No Consumers defined
# if no consumer exists, add new consumer by the following command.
nats con add SEARCH search \
  --filter=search \
  --ack=explicit \
  --max-pending=1000 \
  --wait=-1s \
  --pull \
  --replay=instant \
  --deliver=all \
  --sample=-1 \
  --max-deliver=1
```

## Test local webhook server

* install `ngrok` if you have not installed yet.

```bash
% brew install ngrok --cask
```

* how to use ngrok
  * please sign in to [ngrok dashboard](https://dashboard.ngrok.com/) and get your `authtoken`.

```bash
% ngrok config add-authtoken <your-ngrok-authtoken>
# below is the case webhook server port is 8001.
% ngrok http 8001
# then you can use "Forwarding" url to webhook i.e. ' https://xxxx-xxx-xxx-xx-xx.xx.ngrok.io/events/'
```
