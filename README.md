# Veg Biryani

This is a cheap remake of the awesome project by [Dev Ashar](https://github.com/devashar13/biryani-cli) in Rust.

The original project's description: 
**Biryani** is a tool created to manage dependecies of your github repositories without locally installing any of them . Given the current version of a dependency with a csv of repositories, biryani can tell if the version is greater than or equal to the version specified or not and update it and create a pull request.

At the moment, this project does none of that.

`veg-biryani` can only search for a package in the dependencies section of the given repos.

## Run Locally

Clone the repo and run the following command:
```
cargo run -- filepath package
```
> `filepath` is the path of the file containing the links separated by new line characters and `package` is the name of the package being searched.

## Pls Contribute

1. Fork the repo
2. Make changes to your local repo
3. Open a pull request

## TODO
- [x] Pull and parse the `Cargo.toml` files for all given repos.
- [x] Check if package is a dependency on the project.
- [x] Check if package version matches
- [x] If package version does not match, is it less than or greater than the mentioned version
- [ ] If version not satisfied, update to latest version and create a pull request

I don't like veg biryani...
