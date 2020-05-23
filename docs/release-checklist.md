## Release Checklist

-   Run `cargo update` and review dependency updates. Commit updated
    `Cargo.lock`.
-   Run `cargo outdated` and review semver incompatible updates.
-   Review changes for every crate since the last release.
    If no changes are found, create a release. Else, run the whole process after updating the dependent crates
-   Edit `Cargo.toml` to change the crate version. Run
    `cargo update -p godwit` so that the `Cargo.lock` is updated. Commit the
    changes and create a new signed tag.
-   Wait for CI to finish creating the release. If the release build fails, then
    delete the tag from GitHub, make fixes, re-tag, delete the release and push.
-   Copy the relevant section of the CHANGELOG to the tagged release notes.
-   Run `ci/build-deb` locally and manually upload the deb package to the
    release.
-   Run `cargo publish`.
-   Run `ci/sha256-releases {VERSION} >> pkg/brew/godwit-bin.rb`. Then edit
    `pkg/brew/ripgrep-bin.rb` to update the version number and sha256 hashes.
    Remove extraneous stuff added by `ci/sha256-releases`. Commit changes.
