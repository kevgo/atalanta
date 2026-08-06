# How to make a new Atalanta release

1. Update CHANGELOG.md
2. replace all occurrences of `0.0.1` with the new version and commit to `main`
3. start the release process:

   ```sh
   git tag v0.0.1 && git push --tags
   ```
