## Release Checklist

-   Run `cargo update` and review dependency updates.
-   Run `cargo outdated` and review semver incompatible updates.
-   Run `cargo deadlinks` and update dead links if any.
-   Review changes for every dependency crate since the last release.
    If no changes are found, create a release. Else, run the whole process after updating the dependent crates.
-   Edit `Cargo.toml`, `PKGBUILD` and `pkg/brew/godwit-bin.rb` to change the version. Run `cargo update -p godwit` to reflect in local `Cargo-lock.toml` and create a new signed tag.
-   Wait for Travis and GitHub Actions to finish CI tests and  creating the release. If the release build fails, then, delete the tag from GitHub, make fixes, re-tag, delete the release and push.
-   Copy the relevant section of the CHANGELOG to the tagged release notes.
-   Run `scripts/release-pkg` locally and manually upload the Debian and Arch package to the release.
-   Check if Travis published the new version. If not, check if tests failed for Travis. In case of unresolvable issue, manually run `cargo publish`.
