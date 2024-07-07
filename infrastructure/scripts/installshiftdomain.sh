sudo cp /usr/src/myapp/infrastructure/scripts/domainshift.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable rtdomainshift.service
sudo service rtdomainshift.service start

sudo /usr/src/myapp/target/release/webserver