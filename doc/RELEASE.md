# How to make a new Atalanta release

1. replace all occurrences of `0.0.1` with the new version and commit to `main`
2. tag the new version in the codebase:

       git tag v0.0.0
3. push the new tag:

       git push --tags
4. verify that the release notes match CHANGELOG.md
5. publish the release
