# testrpc
simple rust server to to test ESP8266 jsonrps 

# local net interface on IP 192.168.0.102 test cli
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "get_payload", "id":123 }' 192.168.0.102:9933

# expected response
{"jsonrpc":"2.0","result":["0xc409…1bb0",0,1,0,0,1],"id":123}
