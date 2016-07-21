#!/bin/bash

#bail out if installed
#bail out if not ./serabis_agent or ./plugins/

if [ ! -e ./serabis_agent ]; then
   echo "./serabis_agent is none existant. have you ran \`cargo build --release\` in the agent dir?" 1>&2
   exit 1
fi

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root" 1>&2
   exit 1
fi

if [ ! $1 ]; then
   echo "Please pass account_key as first argument" 1>&2
   exit 1
fi

shortname="$(hostname -s)"
hostname="$(hostname -f)"
account_key="$1"
htauth_user=serabis
htauth_pass=serapis
curl_url="http://$htauth_user:$htauth_pass@api.serabis.com"
base_url=http://api.serabis.com
agent_user=serabis_agent
_installer_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

#install jq
apt-get install jq -y

agent_key=$(curl -s -X POST -H "Content-Type: application/json" -d '{"shortname":"'"$shortname"'", "hostname":"'"$hostname"'", "account_key": "'"$account_key"'"}' "$curl_url/0.01/agents" | jq '.key' -M -r)

mkdir /var/cache/serabis
mkdir -p /usr/lib/serabis/plugins
mkdir -p /etc/serabis/plugins

#make config
echo "account_key='$account_key'" > /etc/serabis/agent.toml
echo "agent_key='$agent_key'" >> /etc/serabis/agent.toml
echo "base_url='$base_url'" >> /etc/serabis/agent.toml
echo "htauth_user='$htauth_user'" >> /etc/serabis/agent.toml
echo "htauth_pass='$htauth_pass'" >> /etc/serabis/agent.toml

# install files
cp -a "$_installer_dir/plugins/*" /usr/lib/serabis/plugins
ln -s /usr/lib/serabis/plugins/*.plugin /etc/serabis/plugins
cp -f "$_installer_dir/serabis_agent" /usr/bin

useradd -MrU -d /etc/serabis $agent_user

chown $agent_user:$agent_user -R /var/cache/serabis /usr/lib/serabis/plugins /etc/serabis
chmod 755 -R /var/cache/serabis /usr/lib/serabis/plugins /etc/serabis

read -r -d '' service <<EOF
[Unit]
After=network.target

[Service]
Restart=always
RestartSec=10
User=$user
Group=$user
WorkingDirectory=/etc/serabis
Environment="RUST_LOG=serabis_agent=info"
ExecStart=/usr/bin/serabis_agent

[Install]
WantedBy=multi-user.target
EOF

echo "$service" > /etc/systemd/system/serabis_agent.service

echo "serabis_agent ALL = NOPASSWD: /usr/lib/serabis/plugins/network_open_ports" > /etc/sudoers.d/99-serabis_agent

systemctl daemon-reload
systemctl enable serabis_agent
systemctl start serabis_agent
