# Deploy dbsync as a service
cd $CNODE_HOME/scripts
wget https://raw.githubusercontent.com/cardano-community/guild-operators/alpha/scripts/cnode-helper-scripts/dbsync.sh
chmod +x dbsync.sh
## Make sure CNODE_NAME env var has been set
dbsync -d # deploy

Overcome bugs while installing node and dbsync
- dbsync.sh need to be download and executed. Note that CNODE_NAME env var should be set (cnode for example)
- any missing scripts file should be downloaded

View sync process node:
cd $CNODE_HOME/scripts
./gLiveView

View dbsync
tail -10f $CNODE_HOME/logs/dbsync.json

View services
systemctl status cnode.service
systemctl status cnode-dbsync.service


# Expose postgres to network
sudo nano /etc/postgresql/13/main/postgres.conf
change listen_address from localhost to *

The same for postgres 14

# Allow inbound connection
sudo nano /etc/postgresql/13/main/pg_hba.conf
# Add this line to the end of file
host    all             all             0.0.0.0/0               md5

# Do the same for the file: sudo nano /etc/postgresql/14/main/pg_hba.conf
host    all             all             0.0.0.0/0               scram-sha-256

List all running services
systemctl --type=service

# open more port ngrok
sudo nano /opt/ngrok/ngrok.yml
systemctl restart ngrok.service

# See log
journalctl -u ngrok.service


# Install gRest
cd ~codekeeper/dbsync
wget https://github.com/PostgREST/postgrest/releases/download/v9.0.0/postgrest-v9.0.0-linux-static-x64.tar.xz

mkdir postgREST
tar Jxf postgrest*

```bash
# Go to postgres cli
psql cexplorer

create role web_anon nologin;

grant usage on schema public to web_anon;

alter default privileges in schema public grant select on tables to web_anon;

create role authenticator noinherit login password 'postgres';

grant web_anon to authenticator;

# Allow permission to access tables
grant select on all tables in schema public to web_anon;
```

```bash
cd postgREST
nano api.conf
# Put following content in api.conf
# db-uri="postgres://authenticator:postgres@localhost:5432/cexplorer"
# db-schema="public"
# db-anon-role="web_anon"

# Start postgREST with config
./postgrest api.conf
```
