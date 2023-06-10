#!/bin/bash
token=""

# extract version from Cargo.toml
version=$(grep -m 1 '^version' Cargo.toml | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')
echo "Program version: $version"

# Run cargo build with the --release flag
cargo build --release
cd target/release/

app_name="writeme"

# {REPO_NAME}-{VERSION}-{OPERATING_SYSTEM}-{ARCHITECTURE}.tar.gz
target_darwin_arm64="$app_name-$version-darwin-arm64.tar.gz"
file="$target_darwin_arm64"


# Create the archive
tar -czf "$target_darwin_arm64" "$app_name"
echo "Created $target_darwin_arm64"

# github release

# requires curl and jq on PATH: https://stedolan.github.io/jq/

# create a new release 
# user: user's name 
# repo: the repo's name
# token: github api user token
# tag: name of the tag pushed 
create_release() {
    user=$1
    repo="writeme"
    tag="v$version"

    command="curl -s -o release.json -w '%{http_code}' \
        --request POST \
        --header 'authorization: Bearer ${token}' \
        --header 'content-type: application/json' \
        --data '{\"tag_name\": \"${tag}\"}' \
        https://api.github.com/repos/$user/$repo/releases"
    http_code=`eval $command`
    if [ $http_code == "201" ]; then
        echo "created release:"
        cat release.json
        upload_release_file
    else
        echo "create release failed with code '$http_code':"
        cat release.json
        echo "command:"
        echo $command
        return 1
    fi
}

# upload a release file. 
# this must be called only after a successful create_release, as create_release saves 
# the json response in release.json. 
# token: github api user token
# file: path to the asset file to upload 
# name: name to use for the uploaded asset
upload_release_file() {
    
    name=$target_darwin_arm64

    url=`jq -r .upload_url release.json | cut -d{ -f'1'`
    command="\
    curl -s -o upload.json -w '%{http_code}' \
        --request POST \
        --header 'authorization: Bearer ${token}' \
        --header 'Content-Type: application/octet-stream' \
        --data-binary @\"${file}\"
        ${url}?name=${name}"
    http_code=`eval $command`
    if [ $http_code == "201" ]; then
        echo "asset $name uploaded:"
        jq -r .browser_download_url upload.json
    else
        echo "upload failed with code '$http_code':"
        cat upload.json
        echo "command:"
        echo $command
        return 1
    fi
}

create_release "writeme-project"