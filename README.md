serapis-api
===========

Serapis API provider.


Add new agent:
  curl -i -X POST -H "Content-Type: application/json" -d '{"shortname":"test_server", "hostname":"test_server2.lwtn.org"}' http://dev.serapis:8000/0.01/agents

Add data points:
   curl -i -X POST -H "Content-Type: application/json" -d '[{"type":"system.uptime","data1":"test1", "data2":true},{"type":"system.memory.free","data1":"test1", "data2":true}]' http://dev.serapis:8000/0.01/data_items/5c7fce74-0e46-420f-8576-0c4bd785f095

 
