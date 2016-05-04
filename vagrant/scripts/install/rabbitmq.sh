if [ ! -e /etc/apt/sources.list.d/rabbitmq.list ]; then
    echo "deb http://www.rabbitmq.com/debian/ testing main" | sudo tee /etc/apt/sources.list.d/rabbitmq.list
    curl -s https://www.rabbitmq.com/rabbitmq-signing-key-public.asc | sudo apt-key add -
    update_repo rabbitmq

    sudo apt-get install rabbitmq-server -y

    sudo cp /vagrant/vagrant/conf/rabbitmq.config /etc/rabbitmq/
    sudo systemctl restart rabbitmq-server

    sudo rabbitmqctl add_user admin reallysecure
    sudo rabbitmqctl set_user_tags admin administrator
    sudo rabbitmqctl delete_user guest

    sudo rabbitmqctl set_permissions -p / admin ".*" ".*" ".*"
    sudo rabbitmq-plugins enable rabbitmq_management
fi
