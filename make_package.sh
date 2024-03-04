#!/bin/bash


rm *.zip

BIN_NAME="elisam_dev"
BIN_FOLDER="target/release/"
cargo build --release
TMP_FOLDER=$(mktemp -d)
cp $BIN_FOLDER/$BIN_NAME $TMP_FOLDER
cp -r blog $TMP_FOLDER
cp -r static $TMP_FOLDER
cp -r systemd $TMP_FOLDER
cp -r templates $TMP_FOLDER
cp -r resume $TMP_FOLDER
cp -r caddy $TMP_FOLDER
cp *.sh $TMP_FOLDER
cd $TMP_FOLDER
ZIP_FILE_NAME="elisam_dev-$(date +'%b-%d-%Y-%I-%M%p').zip"
zip -r  $ZIP_FILE_NAME .
cd -
mv $TMP_FOLDER/$ZIP_FILE_NAME .
echo "Created project archive $ZIP_FILE_NAME in current directory."
echo "Simply copy to server, unzip, and use:"
echo "systemctl --user restart run_site run_caddy"