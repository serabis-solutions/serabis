VAGRANTFILE_API_VERSION = "2"

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|
    config.vm.box = "bento/debian-8.5"

    config.vm.provider "virtualbox" do |v|
        v.memory = 2048
        v.customize ["modifyvm", :id, "--pagefusion", "on"]
        v.linked_clone = true
    end

    config.vm.hostname = "dev.serabis.com"
    config.ssh.forward_agent = true
    config.hostmanager.enabled = true
    config.hostmanager.manage_host = true
    config.vm.network "private_network", ip: "192.168.55.21"
    config.vm.synced_folder "../", "/mnt/stuff"

    config.vm.provision "shell",
        path: "vagrant/setup.sh",
        privileged: false
end
