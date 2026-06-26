# FIsh Tooling for laZines

Here are my personal "scripts" or automation scripts to avoid doing repetitive stuff.

I rather spend 2 hours automating something, than spend more than 2 minutes doing it every time I need it

fitz will include the following small programs:

- [ ] Name validator
- [ ] File gatherer by extension

## Name validator

This small program will enforce a naming convention for files and directories, default to kebab-case, but with support for PascalCase and snake_case, in case thats needed

Features

| command | description |
| --- | ----------- |
| audit | Checks for naming mistakes only |
| soft | Remove atrocities like `spaces` and (god forbids) `emojis`, and anything that isn't a alphanumeric, `-` or `_` character |
| fix | Automatically enforce a convention |

This feature will include general rules for specific names regardless of the naming convention used, for example:

`readme.md` -> `README.md`  
`cargo.toml` -> `Cargo.toml`  
`License.md` -> `LICENSE.md`  

to name a few

## File gatherer by extension

Search a directory for all the files with a extension and moves them into a folder

## Notes
