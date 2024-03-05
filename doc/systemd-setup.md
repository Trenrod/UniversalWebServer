# Systemd setup

## Prerequirements

- Make sure user `universal_webserver` has read access to all files in `/srv/public/`
```sh
# Create user without home and no login
sudo useradd -r -s /bin/false universal_webserver
# Give read access to all files
sudo chown universal_webserver:universal_webserver /srv/public/* 
sudo chmod 400 /srv/public/* 
```

- Link executable
```sh
sudo cp /home/trenrod/dev/UniversalWebServer/target/debug/universal_webserver /usr/local/bin/universal_webserver
```

- Make sure service can access certificate
```sh
sudo chown universal_webserver /etc/certificates/key.pem
```

- Create and store following content in `/etc/systemd/system/universal_webserver.service`
```ini
[Unit]
Description=Provides http access to /srv/public
After=network-online.target

[Service]
Type=simple
User=universal_webserver
ExecStart=/usr/local/bin/universal_webserver
Restart=always

[Install]
WantedBy=multi-user.target
```
