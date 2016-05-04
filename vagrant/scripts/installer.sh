if [ ! -e ~/.bash_profile ]; then
    echo '[[ -f ~/.bashrc ]] && . ~/.bashrc' > ~/.bash_profile
fi

sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get -y -o Dpkg::Options::="--force-confdef" -o Dpkg::Options::="--force-confold" upgrade -y

function install() {
    . "/vagrant/vagrant/scripts/install/$1.sh"
}

#updates only the provided repo
function update_repo() {
    sudo apt-get update -o Dir::Etc::sourcelist="sources.list.d/$1.list" \
        -o Dir::Etc::sourceparts="-" -o APT::Get::List-Cleanup="0"
}
