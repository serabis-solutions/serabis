function setup_postgres {
    postgres_version=9.5
    restart_required=0

    if [ ! -e /etc/apt/sources.list.d/postgres.list ]; then
        sudo apt-get install -y apt-transport-https

        curl -s https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

        echo 'deb http://apt.postgresql.org/pub/repos/apt/ jessie-pgdg main' | sudo tee /etc/apt/sources.list.d/postgres.list

        update_repo postgres

        sudo apt-get install -y postgresql-$postgres_version postgresql-server-dev-$postgres_version
    fi

    #TODO open postgres on 0.0.0.0

    if grep -q "timezone = 'GMT'" /etc/postgresql/$postgres_version/main/postgresql.conf; then
        sudo sed -i  "s/timezone = 'GMT'/timezone = 'UTC'/" /etc/postgresql/$postgres_version/main/postgresql.conf
        restart_required=1
    fi

    if ! fgrep -q "logging_collector = on" /etc/postgresql/$postgres_version/main/postgresql.conf; then
        echo 'logging_collector = on' | sudo tee -a /etc/postgresql/$postgres_version/main/postgresql.conf
        restart_required=1
    fi

    if ! fgrep -q "log_directory = '/var/log/postgresql'" /etc/postgresql/$postgres_version/main/postgresql.conf; then
        echo "log_directory = '/var/log/postgresql'" | sudo tee -a /etc/postgresql/$postgres_version/main/postgresql.conf
        restart_required=1
    fi

    if ! fgrep -q "log_statement = 'all'" /etc/postgresql/$postgres_version/main/postgresql.conf; then
        echo "log_statement = 'all'" | sudo tee -a /etc/postgresql/$postgres_version/main/postgresql.conf
        restart_required=1
    fi

    if [ $restart_required == 1 ]; then
        sudo systemctl restart postgresql
    fi
}

setup_postgres
