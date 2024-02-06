echo "Killing pulpo if running"
killall pulpo

echo "Creating folders and copying files"
sudo mkdir -p /opt/pulpo/resources
sudo mkdir -p /opt/pulpo/config
sudo cp target/release/pulpo /opt/pulpo/
sudo cp pulpo.desktop /opt/pulpo/
sudo cp resources/* /opt/pulpo/resources/
sudo touch /opt/pulpo/pulpo.log

read -r -p "Install default configuration file (pulpo.conf)? [y/N] " response
if [ "$response" = "y" ] || [ "$response" = "Y" ]; then
        sudo cp config/pulpo.conf /opt/pulpo/config/
		sudo cp config/pulpo.conf /etc/
		echo "ATENTION:"
		echo "Please configure file /etc/pulpo.conf with your proper values"
else
		echo "Skipped"
fi

echo "Setting permissions at /opt/pulpo"
sudo chmod 755 /etc/pulpo.conf
sudo chown root:users /etc/pulpo.conf

sudo chmod -R 755 /opt/pulpo
sudo chown -R root:users /opt/pulpo

sudo chmod 777 /opt/pulpo/pulpo.log
sudo chown root:users /opt/pulpo/pulpo.log

echo "Creating desktop-app and updating desktop-database"
sudo desktop-file-install --dir=$HOME/.local/share/applications /opt/pulpo/pulpo.desktop
sudo update-desktop-database $HOME/.local/share/applications
echo "End"
