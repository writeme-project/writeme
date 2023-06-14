#!/bin/bash
token=""

app_name="writeme"

# extract version from Cargo.toml
version=`grep -m1 "version" Cargo.toml | sed 's/version = //' | sed 's/"//g'`
echo "Program version: $version"

# Run cargo build with the --release flag
cargo build --release
path="target/release"
cd $path
target_darwin_arm64="$app_name-$version-darwin-arm64.tar.gz"
tar -czf "$target_darwin_arm64" "$app_name"
echo "Created $target_darwin_arm64"
cd ../..

# {REPO_NAME}-{VERSION}-{OPERATING_SYSTEM}-{ARCHITECTURE}.tar.gz

cargo build --release --target=x86_64-apple-darwin
path_x86="target/x86_64-apple-darwin/release"
cd $path_x86
target_darwin_amd64="$app_name-$version-darwin-amd64.tar.gz"
tar -czf "$target_darwin_amd64" "$app_name"
echo "Created $target_darwin_amd64"
cd ../../..

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

    # Get the previous release version
    previous_tag=$(git describe --abbrev=0 --tags)
    
    # Generate the changelog URL
    changelog_url="https://github.com/$user/$repo/compare/$previous_tag...$tag"
    
    # Create the release body with the changelog URL
    body="**Full Changelog**: $changelog_url"

    command="curl -s -w '%{http_code}' \
        --request POST \
        --header 'authorization: Bearer ${token}' \
        --header 'content-type: application/json' \
        --data '{\"tag_name\": \"${tag}\", \"body\": \"$body\"}' \
        https://api.github.com/repos/$user/$repo/releases"
    http_code=`eval $command`
    if [ $http_code == "201" ]; then
        echo "created release:"
        cat release.json
        upload_release_file "$path/$target_darwin_arm64" "$target_darwin_arm64"
        upload_release_file "$path_x86/$target_darwin_amd64" "$target_darwin_amd64"

        rm "$path/$target_darwin_arm64"
        rm "$path_x86/$target_darwin_amd64"
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
    file=$1
    name=$2

    url=`jq -r .upload_url release.json | cut -d{ -f'1'`
    command="\
    curl -s -w '%{http_code}' \
        --request POST \
        --header 'authorization: Bearer ${token}' \
        --header 'Content-Type: application/octet-stream' \
        --data-binary '@$file' \
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