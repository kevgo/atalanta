# How to make a new Atalanta release

1. update [CHANGELOG.md](CHANGELOG.md) and commit to `main`
2. replace all occurrences of `0.0.1` with the new version and commit to `main`
3. tag the new version in the codebase:

       git tag v0.0.0
4. push the new tag:

       git push --tags
5. verify that the release notes match CHANGELOG.md
6. publish the release
