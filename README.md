# The Git-Deployer
A tool designed for simple CI/CD pipelines. The program listens for POST requests from a Git server (e.g. GitHub, SourceHut)
and pulls the repository to a specified location on the machine. 

This project is a reimplementation of the [Original Git Deployer](https://git.sr.ht/~signaryk/git-deployer) written in Rust
as part of an ongoing effort of mine to learn Rust.

---
## Curent State
- The program works with both SSH and HTTP+OAuth connections.
- Currently it only target GitHub's Webhook API. The goal of the program is to be platform-agnostic, but we're starting with
  GitHub's Webhook API since that is what got this project going to start with. 
- Some assumptions about the CI/CD workflow are currently made such as:
    - The repository's primary branch is protected from direct commits
    - A new deployment should be made when a merge request is accepted
    - The primary branch is the branch to deploy from

---
# Config File Format
The config file consists of two parts:
1. A list of all repository names which should be handled by the program
2. Per-repository configuration for connection (connection type, credentials)

Sample:

```yaml
repos:
  user1/repo1:
    connection: ssh
    directory: /var/www/repo1
  user1/repo2:
    connection: http
    user: user1
    token: <OAuth token>
    directory: $HOME/src/repo2
```

The repository list should be the full name of the repository, which includes the username on platforms like GitHub, GitLab, and
SourceHut. The repository names in the repos list and the repo configuration block must be identical.

### Repository Configuration Block
- `connection`: value should be either `ssh` or `http`
    - When setting the value to `ssh`, it is assumed that you already have an SSH key configured
    - Currently the program assumes you need to authenticate in order to access repositories over http
    - `user`: username to authenticate as
    - `token`: OAuth token for the user (could also be a password but this is highly discouraged)
- `directory`: path on the disk to store the repository

---
## Future Plans
- [X] Document config file syntax
- [ ] Support other Git hosting platforms (e.g. SourceHut, GitLab)
- [ ] Auto-reload configuration file when changes are made
- [ ] Configure when to fetch a changes (pull requests, any commits to a branch, etc.)
- [ ] Specify branch to use to use in configuration file
