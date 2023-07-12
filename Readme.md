# Install rust in ubuntu
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustc --version
sudo apt update
sudo apt upgrade
sudo apt install build-essential

# also install
sudo apt install librust-openssl-sys-dev
sudo apt install libudev-dev
sudo apt install libssl-dev
sudo apt install pkg-config


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

# Run PG Admin
cd Profolio-rust/environments/
source my_env/bin/activate
gunicorn --bind unix:/tmp/pgadmin4.sock --workers=1 --threads=25 --chdir ~/Profolio-rust/environments/my_env/lib/python3.6/site-packages/pgadmin4 pgAdmin4:app