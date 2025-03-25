# GitHub Release Downloader

## Motivation

Created this tool for my personal needs, but it might be useful for others as well. I use this to download releases from private repositories in production servers.

## Usage

```bash
github-release-downloader --help
```

### Example Usage

Download a release from a public repository:

```bash
github-release-downloader myusername myrepo # will only work if repo is public
```

Download a release from a private repository:

```bash
github-release-downloader myusername myprivaterepo github_pat_...
```

Download release assets to a specific directory:

```bash
github-release-downloader myusername myprivaterepo github_pat_... -o /path/to/output
```

## Generating a Personal Access Token

To create a personal access token, follow these steps:

1. Go to [GitHub Settings](https://github.com/settings/personal-access-tokens).
2. Click on "Generate new token".
3. Give your token a descriptive Token name and description.
4. Select **Only select repositories** and add repositories you need.
5. Under **Respository permissions** -> **Contents** select **Read-only**
6. Click on "Generate token".
7. Copy the token and use it as the third argument when running the command.
