# Install rust in ubuntu
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustc --version
sudo apt-get update
sudo apt-get upgrade
sudo apt install build-essential

# also install
sudo apt install librust-openssl-sys-dev libudev-dev libssl-dev pkg-config


# Install and start postgres in ubuntu
sudo apt update
sudo apt install postgresql postgresql-contrib 
sudo systemctl start postgresql.service

sudo -i -u postgres
psql

CREATE DATABASE database_name;
CREATE  USER user_name;
\password user_name //change password
GRANT ALL PRIVILEGES ON DATABASE "database_name" to my_username;

\l //list db
\c _name /// select db


# install githud cli in ubuntu
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo gpg --dearmor -o /usr/share/keyrings/githubcli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
sudo apt update
sudo apt install gh
gh auth login

# Run Backend
cargo run -- daemon --registry-watcher=disabled

# Install PGAdmin
sudo apt install libgmp3-dev libpq-dev
sudo mkdir -p /var/lib/pgadmin4/sessions
sudo mkdir /var/lib/pgadmin4/storage
sudo mkdir /var/log/pgadmin4
sudo chown -R ubuntu:ubuntu /var/lib/pgadmin4
sudo chown -R ubuntu:ubuntu /var/log/pgadmin4

sudo apt install -y python3-pip
sudo apt install -y python3-venv

mkdir environments
cd environments/
python3 -m venv my_env
source my_env/bin/activate

python -m pip install -U pip
python -m pip install pgadmin4==6.10
python -m pip install gunicorn

# Setup PgAdmin
nano my_env/lib/python3.10/site-packages/pgadmin4/config_local.py 

```
LOG_FILE = '/var/log/pgadmin4/pgadmin4.log'
SQLITE_PATH = '/var/lib/pgadmin4/pgadmin4.db'
SESSION_DB_PATH = '/var/lib/pgadmin4/sessions'
STORAGE_DIR = '/var/lib/pgadmin4/storage'
SERVER_MODE = True
AZURE_CREDENTIAL_CACHE_DIR =  '/var/lib/azurecredentialcache'

```
sudo mkdir /var/lib/azurecredentialcache 
chown -R ubuntu:ubuntu /var/lib/azurecredentialcache
python my_env/lib/python3.10/site-packages/pgadmin4/setup.py

sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 80 -j ACCEPT
# Run PG Admin
gunicorn --bind unix:/tmp/pgadmin4.sock --workers=1 --threads=25 --chdir ~/Profolio-rust/environments/my_env/lib/python3.6/site-packages/pgadmin4 pgAdmin4:app

# Setup Nginx
sudo apt install nginx
cd /etc/nginx/sites-available
sudo nano profoliodb.litehires.com

```

server {
    listen 80;
    server_name profoliodb.litehires.com;

    location / {
        include proxy_params;
        proxy_pass http://unix:/tmp/pgadmin4.sock;
    }
}

```
sudo ln -s /etc/nginx/sites-available/profoliodb.litehires.com /etc/nginx/sites-enabled/
sudo systemctl restart nginx