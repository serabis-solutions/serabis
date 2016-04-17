#install the nodesource node.js repos
if [ ! -e /etc/apt/sources.list.d/nodesource.list ]; then
    #ripped from https://deb.nodesource.com/setup_4.x, i'm not a fan of pipeing random scripts into bash as root
    sudo apt-get install -y apt-transport-https
    curl -s https://deb.nodesource.com/gpgkey/nodesource.gpg.key | sudo apt-key add -
    echo 'deb https://deb.nodesource.com/node_4.x jessie main' | sudo tee /etc/apt/sources.list.d/nodesource.list
    echo 'deb-src https://deb.nodesource.com/node_4.x jessie main' | sudo tee -a /etc/apt/sources.list.d/nodesource.list

    sudo apt-get -qq update

    sudo apt-get install -y build-essential nodejs
fi

if [ ! -e ~/npm ]; then
    mkdir ~/npm

    echo 'NPM_PACKAGES="${HOME}/npm"' >> ~/.bash_profile
    echo 'PATH="$NPM_PACKAGES/bin:$PATH"' >> ~/.bash_profile
    echo 'export MANPATH="$NPM_PACKAGES/share/man:$MANPATH"' >> ~/.bash_profile

    echo prefix=${HOME}/npm >> ~/.npmrc

    source ~/.bash_profile
fi
