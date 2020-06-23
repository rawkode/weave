# Weave

Weave is a build planner and delegator for mono repositories.

## Why?

I ([@rawkode](https://twitter.com/rawkode)) firmly believe that the best build tool is the tool purpose built for the language. However, we seem to be in a world with build tools like Bazel, Buck, and Pants. Often written in Java, have some weird DSL, and try to do too much.

Perhaps I am fortunate enough to not be at the scale to need these tools, whatever that scale may be; but I wanted a build planner that would delegate to the tools I know and are specifically built for their own platforms.

Weave aims to detect and delegate builds to:

-   Cargo for Rust, when it detects a `Cargo.toml`
-   Composer for PHP, when it finds a `composer.json`
-   Docker when there's a `Dockerfile`
-   Go for Go, when it detects whatever package manager is in right-now.
-   Make when there's a `Makefile`
-   GitLab CI when there's a `.gitlab-ci.yml`
-   NPM or Yarn for NodeJS, when it finds a `package.json`

You get the point.

Currently Weave doesn't do all this, but it will soon.

### Roadmap

-   DAG and Build Dependencies
-   Configuration of each build step through `weave.(toml|yaml)`
-   Exporters for CircleCI, GitLab, and GitHub Actions
    -   Though, I believe, only [GitLab CI supports dynamic build plans](https://docs.gitlab.com/ee/ci/parent_child_pipelines.html#dynamic-child-pipelines) ATM

## Contributing

### Observe

Observe modules should be able to detect what has changed and provide a list of Paths that can be passed to detect

#### All

The `all` Observer returns all child directories within the top level directory

#### Git

The `git` Obeerver returns all the modified, by the last commit, directories.

### Detect

Detect will process each of the Paths provided, walking back to the root of the directory tree; returning a list of Paths and the discovered build tooling.

#### Dockerfile

Currently, Dockerfiles are detected and can be built - so they're all built with the same tag: `weave_image` 😂

#### GitLab CI

GitLab CI roots are detected, but not actioned.

#### Makefile

Makefile roots are detected, but not actioned.

### Build

Build will return a build plan, through a DAG, of the steps needed to build the recent changes.

## Icon

Spiderweb icon by [icons8](https://icons8.com/icons/set/spiderweb)
