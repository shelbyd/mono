# mono

Command line utilities for managing mono-repos.

Meant to be used by developers and in CI/CD.

### Usage

Any subdirectory can have a `MONO` file that specifies how that directory relates to the rest
of the project.

#### Sync Files

MONO files can specify files to sync from one location in the project to another.

```yaml
sync_files:
  - from: .github/workflows/*
    to: /.github/workflows/${ dir_slug }-${ file_name }
```

The above config, when located at, `foo/MONO` will copy a file like
`foo/.github/workflows/test.yml` to `.github/workflows/foo-test.yml`.
