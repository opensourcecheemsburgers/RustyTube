# Translating RustyTube

1. Fork RustyTube.
2. Create a new translation file like so `frontend/locales/{language-code}-{variant}.toml`.
   For example, `en-US.toml` for English (US), or `fi-FI.toml` for Finnish (Finland).
3. Copy the `en_US.toml` translations into your new file.
4. Translate all of the values from each key-value pair into your chosen language.
5. Add your language to [`shared/locales/src/lib.rs`](shared/locales/src/lib.rs).
6. Commit the new translation to your fork.
7. Submit a pull request on the official RustyTube github.

Here is an example of a translation [commit](https://github.com/opensourcecheemsburgers/RustyTube/pull/30/commits/92fec0de442db4a1634d8baa5176558add0286e5).
