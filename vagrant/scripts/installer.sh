if [ ! -e ~/.bash_profile ]; then
    echo '[[ -f ~/.bashrc ]] && . ~/.bashrc' > ~/.bash_profile
fi

sudo apt-get update -qq
sudo apt-get upgrade -y

function install() {
    . "/vagrant/vagrant/scripts/install/$1.sh"
}
