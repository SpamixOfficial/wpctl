# Repository

**Don't forget to read the important notes**

This is the documentation for all repository related things, like the repository manfiest and wallpaper manfiests.

## Best practices

A repository should not contain anything else than manifests. If it does you shoul be aware that it will be fetched to every user

## Repository Manifest

This manifest should be installed locally as a manifest file in the config directory.

**It should also be present in the root of the remote as the file `manifest.toml`**

Example file path: `$HOME/.config/wpctl/repositories/default_repository.toml`

**NOTE: The file name can be anything except config.toml really, but using the identifier as the name is preferred, ex. se.spamix.wprepo**

### Fields

**NOTE: There are no defaults, so every value not marked as optional must be explicitly set**

| Field      | Type       | Description                                                                                                                                              |
| ---------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| name       | string     | The name of the repository                                                                                                                               |
| pretty_url | string/url | The "pretty url" of the repository, an url that the user can visit. Can be a website, or a link to a github repository or anything really that is an url |
| git_url    | string/url | The git url, should point to the remote git repository containing wallpaper manifests                                                                    |
| identifier | string     | Unique identifier, ex. `se.spamix.wprepo` or `com.example.coolrepo`                                                                                      |
| version    | string     | Manifest version, used for checking compability                                                                                                          |

## Wallpaper Manifest

In every repository there should be wallpaper manifests. These are what wpctl sees as "packages".

The manifests should use the toml filetype, follow the specification below and be located in the root of the repository (the latter might be subject of change).

### Fields

| Field         | Type              | Description                                                      |
| ------------- | ----------------- | ---------------------------------------------------------------- |
| name          | string            | Name of the package                                              |
| description   | string            | Description of the package                                       |
| author        | [string]          | Author/authors of the wallpaper                                  |
| source        | Opt\<string/url\> | The source of the wallpaper, optional as source may not be known |
| maintainer    | string            | Name of package maintainer                                       |
| thumbnail_url | string/url        | Link to thumbnail_url of package, shown to users in detail panel |
| download_url  | string/url        | Link to package zip or image file                                |
| sizes         | [WpSize]          | What aspect ratios the wallpaper support/work on                 |

For type references see the [README](./README.md)

## Important notes

The download_url must point to an image (not webp) or zip file. In case of multi-image zip file all of the images will be installed under that package name.

The wallpaper manifests may **not** be named `manifest.toml` as this is the repository manifest.
