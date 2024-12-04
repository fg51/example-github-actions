# example-actions

![](https://github.com/kflange/example-actions/workflows/Rust/badge.svg)

example github actions

# view log

at workflow is running.

```sh
$ gh run watch
```

at workflow is completed.

```sh
$ gh run view
```

# os

- ubuntu-latest
- windows-latest
- macos-latest

# pre-installed tools

- lang: node.js, java, python, go, ruby, .net, rust
- package manager: npm, maven, pip, rubygems, nuget
- tools: jq, openssl, docker, kubectl, github cli, aws cli
- browser: selenium, chrome, edge

# actions

![https://github.com/marketplace](https://github.com/marketplace)

# 3.1 context

## github context

```yml
steps:
  - run: echo "${{ github.actor }}"
```

- github.run_id: workflow id
- github.head_ref: source branch of pull-request
- github.workspace: steps's working directory
- github.repository: repository name
- github.repository_owner: repository owner
- github.event: trigger event object

![caution]

```
"${{ github.event.pull_request.title }}"  # event is object
```

## runner context

- runner.name
- runner.os
- runner.arch
- runner.temp: termporary directory's path
- runner.tool_cache: pre-installed tool's direcotry path
- runner.debug

# 3.2 environment variables

```yml
env:
  BRANCH: main
steps:
  - run: echo "${BRANCH}" # in shell command
  - uses: arctions/checkout@v4
    with:
      ref: ${{ env.BRANCH }} # via 'env' context
```

# 3.3 variables

variables is common value through the work flow

## set

a. in web page, from setting menu.
b. in shell, gh variable set USERNAME --body 'octocat'

## useage

```yml
env:
  USERNAME: ${{ vars.USERNAME }}
steps:
  - run: echo "${USERNAME}"
```

# 3.4 secrets

- secrets value is coded in github.
- secrets is masked in log.
- secrets cannot be conformed forever.

## usage

```yml
env:
  PASSWORD: ${{ secrets.PASSWORD }}
steps:
  - run: echo "${PWASSWORD}"
```

# 4.3.1 filter

- paths
- paths-ignore
- branches
- branches-ignore
- tags
- tags-ignore

# 4.3.2 glob

- \*: match charactor exclude slash
- \*\*: match charactor include slash
- ?: match the before charactor (zero charactor or more)
- +: match the before charactor (one charactor or more)