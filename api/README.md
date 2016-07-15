# serapis-api

## first run
```
cd /vagrant/api
npm install
db-migrate --config config/database.json -e dev up
```

## Serapis API provider.

### Add a new account:
```
curl -i -X POST -H "Content-Type: application/json" -d '{"name": "Demo Account", "shortname":"demo"}' http://dev.serapis:8000/0.01/accounts
```

### Add new agent:
```
curl -i -X POST -H "Content-Type: application/json" -d '{"shortname":"test_server", "hostname":"test-server2.lwtn.org", "account_key": "d7f98c53-a06c-4253-86c5-828c9f03ba7a"}' http://dev.serapis:8000/0.01/agents
```

### Add data points:
```
curl -i -X POST -H "Content-Type: application/json" -d '[{"type":"system.uptime","data1":"test1", "data2":true},{"type":"system.memory.free","data1":"test1", "data2":true}]' http://dev.serapis:8000/0.01/data_items/aa25fe1e-32c3-4e9a-ab77-ed625cdc9f44/5c7fce74-0e46-420f-8576-0c4bd785f095   # ${account_key}/${agent_key}
```


### Add conditions to monitor:
```
curl -i -X POST -H "Content-Type: application/json" -d '{ "name": "5min Load above 3 and 1min > 4", "level": 3, "condition": [{ "type": "load", "key": "5min", "value": { ">=": "3" } }, { "type": "load", "key": "1min", "value": { ">=": "4" } }] } ' http://dev.serapis:8000/0.01/conditions/new/4cfd9fd6-3fcf-4aad-8183-67a21c338b22
```
