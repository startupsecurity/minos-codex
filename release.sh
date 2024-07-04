#!/bin/bash

# Ensure the script exits if any command fails
set -e

# Function to get the current version from Cargo.toml
get_version() {
    grep '^version =' Cargo.toml | sed -E 's/version = "(.*)"/\1/'
}

# Function to bump the patch version
bump_patch() {
    IFS='.' read -r -a parts <<< "$1"
    parts[2]=$((parts[2] + 1))
    echo "${parts[0]}.${parts[1]}.${parts[2]}"
}

# Function to bump the minor version
bump_minor() {
    IFS='.' read -r -a parts <<< "$1"
    parts[1]=$((parts[1] + 1))
    parts[2]=0
    echo "${parts[0]}.${parts[1]}.${parts[2]}"
}

# Function to bump the major version
bump_major() {
    IFS='.' read -r -a parts <<< "$1"
    parts[0]=$((parts[0] + 1))
    parts[1]=0
    parts[2]=0
    echo "${parts[0]}.${parts[1]}.${parts[2]}"
}

# Get the current version
current_version=$(get_version)
echo "Current version: $current_version"

# Check the argument for which version to bump
if [ "$1" == "bump_patch" ]; then
    new_version=$(bump_patch "$current_version")
elif [ "$1" == "bump_minor" ]; then
    new_version=$(bump_minor "$current_version")
elif [ "$1" == "bump_major" ]; then
    new_version=$(bump_major "$current_version")
else
    echo "Usage: $0 {bump_patch|bump_minor|bump_major}"
    exit 1
fi

echo "New version: $new_version"

# Update the version in Cargo.toml
sed -i -E "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml

# Commit the changes to Cargo.toml
git add Cargo.toml
git commit -m "Bump version to $new_version"

# Tag the new version
git tag "v$new_version"

# Push the changes and the tags
git push origin main
git push origin "v$new_version"

echo "Version bumped to $new_version and tagged with git."