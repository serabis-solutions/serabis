. /vagrant/vagrant/scripts/installer.sh

install node
install postgres

npm install -g db-migrate
if [[ ! $(sudo -u postgres psql -t -c "SELECT datname FROM pg_database WHERE datname = 'serapis_dev'") ]]; then 
    sudo -u postgres psql -c "CREATE USER serapis WITH PASSWORD 'reallysecure';"
    sudo -u postgres createdb serapis_dev -O serapis
fi

if [ ! -e /sub/sbin/apache2 ]; then
    sudo apt-get -y install apache2
    sudo cp /vagrant/vagrant/conf/serapis-frontend.conf /etc/apache2/sites-enabled/
    sudo a2enmod headers
    sudo systemctl reload apache2
fi

install rabbitmq

sudo rabbitmqctl list_vhosts | grep "^serapis$" -q || {
    sudo rabbitmqctl add_vhost serapis
    #give admin use access to this vhost in the managment interface
    sudo rabbitmqctl set_permissions -p serapis admin ".*" ".*" ".*"
}
sudo rabbitmqctl list_users | grep "^inserter\b" -q || {
    sudo rabbitmqctl add_user inserter reallysecure
    sudo rabbitmqctl set_permissions -p serapis inserter ".*" ".*" ".*"
}
