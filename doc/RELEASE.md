# How to make a new Atalanta release

1. replace all occurrences of `0.0.1` with the new version and commit to `main`
2. start the release process:

   ```
   git tag v0.0.1 && git push --tags
   ```

3. verify that the release notes match CHANGELOG.md
4. publish the release
