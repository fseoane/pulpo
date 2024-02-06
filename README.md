# pulpo
A gnome shell (wayland) notifier for Gotify and/or Ntfy server.

Gotify/Ntfy server is open source software for notifications that can be deployed on-prem (self-hosted). 
It provides web based service, API,  and also mobile application, but I was missing a working gnome tray notification app for receiving those notifications directly in my gnome desktop (wayland), so I build this.

## 1.Requirements
Built on Rust

It also requires "Ayatana AppIndicator3" and Gnome shell extension "AppIndicator and KStatusNotifierItem Support".

The Gnome shell extension "AppIndicator and KStatusNotifierItem Support" can be installed from : https://extensions.gnome.org/extension/615/appindicator-support/



### 2.Installation

### 2.1.Installing the release package (precompiled)

#### 2.1.A.Installing the prerequisites for running the package
Install the prerequisites for your distribution:

- Arch :

        sudo pacman -S libappindicator-gtk3
  

#### 2.1.B.Installing the precompiled ZIP release package

Download the precompiled release package ZIP file for your distribution and copy it's contents to /opt/pulpo. 
This release package has a binary already compiled and ready to execute on Linux.

The bash script (install.sh) inside the realease package will copy all the necessary files to /opt/pulpo in one go.....

....just edit the file pulpo.conf in /etc (default location is /etc) and set the proper values Gotify and/or Ntfy server:
- gotify server url, and 
- gotify client token  (you may need to generate this in your Gotify server)
- ntfy server url and port
- ntfy topics to be subscribed

pulpo.conf file looks like this

                [config]
                tray_icon=""pulpo-green-64x64.png"

                [gotify]
                gotify_url="http(s)://gotify-host:port"
                gotify_client_token="gotify-client-token"
                gotify_sound="notification.ogg"
                gotify_icon="gotify-icon.png"

                [ntfy]
                ntfy_url="http(s)://ntfy-host:port"
                ntfy_topics="Topic1,Topic2,Topic3,Topic4,...,topicN"
                ntfy_sound="notification.ogg"
                ntfy_icon="ntfy-icon.png"     

Note: You can configure only Gotify, only Ntfy, or both at the same time.

### 2.2.Installing from AUR (Arch Linux)
To install the application from AUR (Arch User Repo) you can use an AUR helper like yay and execute:
                
                yay -S pulpo-bin



