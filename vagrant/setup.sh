. /vagrant/vagrant/scripts/installer.sh

install node
install postgres

npm install -g db-migrate
if [[ ! $(sudo -u postgres psql -t -c "SELECT datname FROM pg_database WHERE datname = 'serapis_dev'") ]]; then 
    sudo -u postgres psql -c "CREATE USER serapis WITH PASSWORD 'reallysecure';"
    sudo -u postgres createdb serapis_dev -O serapis
fi

sudo apt-get -y install apache2
sudo cp /vagrant/vagrant/conf/serapis-frontend.conf /etc/apache2/sites-enabled/
sudo a2enmod headers
sudo systemctl reload apache2
