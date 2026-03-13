# Distribution Guide

This guide covers CLI distribution for Homebrew (custom tap) and Windows winget.

## Release assets

The release workflow publishes these binary assets to GitHub Releases:

- `krxon-x86_64-unknown-linux-gnu.tar.gz`
- `krxon-x86_64-apple-darwin.tar.gz`
- `krxon-aarch64-apple-darwin.tar.gz`
- `krxon-x86_64-pc-windows-msvc.zip`
- `checksums.txt`

## Homebrew (custom tap)

Use this repository as the Homebrew tap:

- Repository: `github.com/seungdols/krxon`
- Formula path: `Formula/krxon.rb`

Use `packaging/homebrew/krxon.rb` in this repo as template.

### Update formula for a new release

1. Compute source tarball SHA256 for `vX.Y.Z`:

```bash
VERSION=0.1.2
curl -L "https://github.com/seungdols/krxon/archive/refs/tags/v${VERSION}.tar.gz" -o /tmp/krxon-src.tgz
shasum -a 256 /tmp/krxon-src.tgz
```

2. Replace placeholders in formula:

- `__VERSION__` -> release version (without `v`)
- `__SOURCE_SHA256__` -> SHA256 from step 1

3. Commit formula update in `krxon` and push.

### Install command for users

```bash
brew tap seungdols/krxon
brew install krxon
```

## winget

Use `packaging/winget/*.yaml` as templates.

### Update manifests for a new release

1. Download `checksums.txt` from GitHub Release `vX.Y.Z`.
2. Find SHA256 for `krxon-x86_64-pc-windows-msvc.zip`.
3. Replace placeholders:

- `__VERSION__` -> release version (without `v`)
- `__WINDOWS_X64_SHA256__` -> checksum from `checksums.txt`

4. Add manifests to `winget-pkgs` under:

`manifests/s/seungdols/krxon/<VERSION>/`

5. Open PR to `microsoft/winget-pkgs`.

### Install command for users

```powershell
winget install seungdols.krxon
```
