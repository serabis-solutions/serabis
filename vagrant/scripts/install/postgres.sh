if [ ! -e /etc/apt/sources.list.d/postgres.list ]; then
    sudo apt-get install -y apt-transport-https

    curl -s https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

    echo 'deb http://apt.postgresql.org/pub/repos/apt/ jessie-pgdg main' | sudo tee /etc/apt/sources.list.d/postgres.list

    update_repo postgres

    postgres_version=9.5
    sudo apt-get install -y postgresql-$postgres_version postgresql-server-dev-$postgres_version

    if grep -q "timezone = 'GMT'" /etc/postgresql/$postgres_version/main/postgresql.conf; then
        sudo sed -i  "s/timezone = 'GMT'/timezone = 'UTC'/" /etc/postgresql/$postgres_version/main/postgresql.conf
        sudo systemctl restart postgresql
    fi

    #open postgres on 0.0.0.0
fi
