run `cargo build --release` on a debian box

* copy agent-installer/installer.sh   => $puppet/serabis_agent/files/installer/installer.sh
* copy agent-plugins                  => $puppet/serabis_agent/files/installer/plugins
* copy target/release/serabis-agent   => $puppet/serabis_agent/files/installer/serabis/serbis_agent

so your puppet module dir looks like this

```mark@h1:/etc/puppet$ ls -l serabis_agent/files/installer
total 3452
-rwxr-xr-x 1 mark mark    2294 Jul 21 16:45 installer.sh
drwxr-xr-x 3 mark mark    4096 Jul 21 16:35 plugins
-rwxr-xr-x 1 mark mark 3523296 Jul 21 16:35 serabis_agent```

$puppet/serabis_agent/manifests/init.pp

```class serabis_agent ( $account_key ) {
  file { "/tmp/serabis_installer":
    ensure  => "directory",
    source  => "puppet:///modules/serabis_agent/installer",
    recurse => true,
    owner   => "root",
    group   => "root",
  }
  exec { "install serabis_agent":
    command => "/bin/bash /tmp/serabis_installer/installer.sh $account_key",
    creates => "/usr/bin/serabis_agent",
    require => File['/tmp/serabis_installer'],
  }
}```
