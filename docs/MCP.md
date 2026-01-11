# MCP (Model Context Protocol) for Rustwork

Serveur JSON-RPC 2.0 pour l'intégration IDE. Permet d'interroger les métadonnées d'un projet Rustwork de manière sécurisée.

## Quick Start

```bash
# Démarrer le serveur
rustwork mcp

# Ou avec options
rustwork mcp --host 127.0.0.1 --port 4000 --project /path/to/project
```

## Méthodes Disponibles

| Méthode | Description |
|---------|-------------|
| `get_manifest` | Contenu du `.rustwork/manifest.json` (secrets masqués) |
| `get_conventions` | Conventions du framework (erreurs, handlers, middleware) |
| `get_routes` | Routes enregistrées |
| `get_models` | Modèles enregistrés |
| `get_project_info` | Métadonnées du projet (nom, version, database) |

## Exemples

### Python

```python
import socket, json

def query(method):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(("127.0.0.1", 4000))
    request = {"jsonrpc":"2.0", "id":1, "method":method, "params":{}}
    sock.sendall((json.dumps(request) + "\n").encode())
    response = sock.recv(4096).decode()
    sock.close()
    return json.loads(response)

# Usage
result = query("get_project_info")
print(result["result"]["project_name"])
```

Script complet: [examples/mcp_client.py](../examples/mcp_client.py)

### Shell (netcat)

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"get_manifest","params":{}}' | nc 127.0.0.1 4000
```

### Node.js

```javascript
const net = require('net');
function sendRequest(method) {
  return new Promise((resolve, reject) => {
    const client = net.connect({ host: '127.0.0.1', port: 4000 }, () => {
      client.write(JSON.stringify({jsonrpc:'2.0',id:1,method,params:{}}) + '\n');
    });
    client.on('data', (data) => { resolve(JSON.parse(data.toString())); client.end(); });
    client.on('error', reject);
  });
}
```

## Réponses

### Succès
```json
{"jsonrpc":"2.0","id":1,"result":{"project_name":"my-api","rustwork_version":"0.1.0"}}
```

### Erreur
```json
{"jsonrpc":"2.0","id":1,"error":{"code":-32603,"message":"manifest not found"}}
```

**Codes d'erreur:** -32600 (Invalid Request), -32601 (Method Not Found), -32602 (Invalid Params), -32603 (Internal Error)

## Sécurité

✅ Bind localhost (127.0.0.1) uniquement  
✅ Secrets masqués automatiquement (password/token/key → ***)  
✅ Lecture seule (pas d'exécution de code)  
✅ Limite 1MB par requête
