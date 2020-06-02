[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-green.svg)](https://conventionalcommits.org)

[![Build Status](http://192.168.2.69:4000/api/badges/Fice/calpipe/status.svg)](http://192.168.2.69:4000/Fice/calpipe)

#GIT COMMIT MESSAGES
This project uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0-beta.4/). This makes sure we can auto generate CHANGELOGS via TODO: . It also makes reading the git history easier and enables future 3rd party tools to automate changelogs and more.

If you wasnt to make sure your commit are correct (correct git message format,  code linting, test running), then run
```./setup-git.sh``` to install the necessary hooks.

#RELEASING
Steps todo before releaseing:
1. Update changelog: 
    ```git journal -a > CHANGELOG.md```



#Help with the book

Install [mdBook](https://github.com/rust-lang/mdBook) and start editing the files in book/