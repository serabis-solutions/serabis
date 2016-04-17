if grep -q "timezone = 'GMT'" /etc/postgresql/9.4/main/postgresql.conf; then
    sudo sed -i  "s/timezone = 'GMT'/timezone = 'UTC'/" /etc/postgresql/9.4/main/postgresql.conf
    sudo systemctl restart postgresql
fi

sudo apt-get install -y postgresql-9.4 postgresql-server-dev-9.4
