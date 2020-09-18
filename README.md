# API - CRUD controle agua para bot telegram

## iptables
```zsh
sudo iptables -A INPUT -p tcp --dport 8000 -j ACCEPT
```

## cargo
```zsh
bash -c "RUST_BACKTRACE=full cargo run"

cargo build 

cargo run
🛰  Mounting /consumos:
    => GET /consumos (get_consumos)
    => GET /consumos/<idkey> (get_consumo)
    => POST /consumos (create_consumo)
    => DELETE /consumos/<idkey> (delete_consumo)
🚀 Rocket has launched from http://0.0.0.0:8000
```

```zsh
rustup update
rustup default stable
rustup default nightly
rustup override set nightly

rustup update && cargo update
```

# cargo run
```zsh
curl -X GET http://localhost:8000/consumos 
curl -X GET http://localhost:8000/consumos/<idkey>

curl -X DELETE http://localhost:8000/consumos/<idkey>


curl -X POST http://192.168.0.46:8000/consumos
{
  "idkey": "243a3c43be545ad3f89fa3c1987d",
  "host": "dev",
  "relogio": 2000,
  "consumo": 100,
  "lavouroupa": false,
  "quantidade": 0,
  "observacao": "registro inicial",
  "diffdias": 1,
  "difftime": 96545,
  "data_cadastro": "17/08/2020",
  "data_alteracao": "17/08/2020"
}
```


# referencias
```txt
https://api.rocket.rs/v0.4/rocket/
https://api.rocket.rs/v0.4/rocket/config/index.html
```

# tabela

