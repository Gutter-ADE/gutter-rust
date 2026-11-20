# Rust for Gutter

Adds hover docs and go to definition for Rust (`.rs`) through [rust-analyzer](https://rust-analyzer.github.io/). Coloring and the palette's symbol list are Gutter's own and don't change.

## Install

- **Browse:** Settings > Extensions > Browse, find this extension, and choose Install. After the confirmation, the rust-analyzer download (pinned release, checked by SHA-256) runs once.
- **By name:** Settings > Extensions > Install from npm…, then the package name.
- **Development folder:** Settings > Extensions > Add development folder… and pick this folder, then choose **Install server** on the rust-analyzer row.

Downloads exist for Windows, macOS, and Linux, on x64 and arm64. The server starts when a Rust file opens and stops after it has been idle. `checkOnSave` is off, so it never runs `cargo check` on its own.

## Use a rust-analyzer already on your PATH

Under this extension's row in Settings, set **rust-analyzer** to `rust-analyzer` (for example rustup's). Gutter then runs that one and the downloaded copy isn't needed.

## Check it

From a checkout of Gutter:

```sh
npm run check-extension -- path/to/gutter-rust
```

It validates `gutter-extension.json`, including each download's URL and SHA-256, and checks what `npm pack` would publish. It doesn't download anything. `samples/` is there to open in Gutter and try hover and go to definition.

## Updating rust-analyzer

Pick a release on rust-analyzer's releases page, change the tag in all six URLs, and replace each `sha256` with the hash of the file you downloaded (`Get-FileHash -Algorithm SHA256 <file>` on Windows, `shasum -a 256 <file>` on macOS and Linux). Then bump `version` in both `package.json` and `gutter-extension.json`; installed copies download the new server when they update.

## Publishing

`package.json` uses the placeholder scope `@your-scope`. Swap in your own, add a `repository` field pointing at this repository's remote, keep `version` the same in `package.json` and `gutter-extension.json`, check it as above, and run `npm publish --access public` from this folder.
