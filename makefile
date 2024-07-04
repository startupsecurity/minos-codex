.PHONY: bump_patch bump_minor bump_major

# Variables
RELEASE_SCRIPT = ./release.sh

# Jobs
bump_patch:
    @$(RELEASE_SCRIPT) bump_patch

bump_minor:
    @$(RELEASE_SCRIPT) bump_minor

bump_major:
    @$(RELEASE_SCRIPT) bump_major