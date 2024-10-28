#!/bin/bash

# input your credentials here!!!
SSID = "ssid"
DEVICE_NAME = "wlan0"
USERNAME = "username"
PASSWORD = "password"

nmcli con add type wifi ifname "$DEVICE_NAME" con-name "$SSID" ssid "$SSID"
nmcli connection modify "$SSID" wifi-sec.key-mgmt wpa-eap 802-1x.eap peap 802-1x.phase2-auth mschapv2 802-1x.identity "$USERNAME" 802-1x.password "$PASSWORD"
nmcli connection up "$SSID"
nmcli connection modify "$SSID" connection.autoconnect yes





