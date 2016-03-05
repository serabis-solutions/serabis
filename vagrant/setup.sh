sudo apt-get install -y apt-transport-https

#install the nodesource node.js repos
#ripped from https://deb.nodesource.com/setup_5.x, i'm not a fan of pipping random scripts into bash as root

curl -s https://deb.nodesource.com/gpgkey/nodesource.gpg.key | sudo apt-key add -
echo 'deb https://deb.nodesource.com/node_5.x jessie main' | sudo tee /etc/apt/sources.list.d/nodesource.list
echo 'deb-src https://deb.nodesource.com/node_5.x jessie main' | sudo tee -a /etc/apt/sources.list.d/nodesource.list

#i should check if update has ran already today, but that's more work and update is quick enough
sudo apt-get update

sudo apt-get install -y postgresql-9.4 postgresql-server-dev-9.4 build-essential nodejs
if [[ ! $(sudo -u postgres psql -t -c "SELECT datname FROM pg_database WHERE datname = 'serapis_dev'") ]]; then 
    sudo -u postgres psql -c "CREATE USER serapis WITH PASSWORD 'reallysecure';"
    sudo -u postgres createdb serapis_dev -O serapis
fi

sudo apt-get -y install apache2
sudo cp /vagrant/vagrant/conf/serapis-frontend.conf /etc/apache2/sites-enabled/
sudo a2enmod headers
sudo systemctl reload apache2
