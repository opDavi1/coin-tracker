# Coin Tracker

A desktop app made to locally host your coin collection in a sqlite database. Add, remove, update, and even import coins straight from numista using the numista api.

As of right now, there is no way to manually add coins to the database. This is coming very soon. If you want to add coins to the database, run test.py to add an example coin to the database, or use it as a reference to add your coins to the collection, although i don't know why you'd do that with the current state of the program. 

## Database Notice
This is a VERY VERY VERY early version of this project. The database file, format, schema, etc. are very volatile and are subject to breaking changes at any point. This will continue until v1.0.0 releases, at which point the database should not recieve breaking changes. I will try not to break the database in these early versions, but be careful of changes that may affect the database if you're using it (again, i dont know why you would be right now)


## License

Coin Tracker is distributed under the GPL-3.0-or-later license. See the included LICENSE.md file or go to [the GNU website](https://www.gnu.org/licenses/licenses.html) for more details.

## Installation

This project is still very early in development. So the best way to "install" Coin Tracker is to just clone this repo, and install the requirements in `requirements.txt`. Once the requirements are installed run the program by navigating to the src directory and running 
```console
$ python main.py
```
Eventually there will probably be an install script of some sort, but that's a long way down the road from now


## Contributing

Thanks for taking the time to contribute!

> ### Legal Notice
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project licence.

### Bug Reports
- Make sure that a similar issue isn't already open
- [Open an issue](https://github.com/opDavi1/coin-tracker/issues/new) for your bug
    - Include as many relevant details as possible in your issue to avoid confusion and help us determine how to fix the bug

### Feature Suggestions
- Make sure that a similar feature isn't already suggested
- [Open an issue](https://github.com/opDavi1/coin-tracker/issues/new) for your feature request

### Your First Code Contribution
To contribute actual sections of code, setup your environment as follows:

1. Fork this repository, clone it, and cd into it
2. Make a python virtual environment using 
```console
$ python -m venv .venv
```
and activate it using 
```console
$ source .venv/bin/activate
```
It is important to name the virtual environment `.venv` because this is included in the .gitignore for the project.
You don't have to name the virtual environment `.venv`, but make sure you don't accidentally include the virtual environment folder in your commits.

3. Install the requirements using 
```console
$ pip install -r requirements.txt
```
4. Make a new branch for your feature or fix
5. Commit some changes, preferably using [Conventional Commit messages](https://www.conventionalcommits.org/en/v1.0.0/#summary)
6. Once your feature or fix is finished, push your changes to github and make a pull request on this project
7. If your change gets accepted, it will be merged into the official project

To deactivate the virtual environment, simply run 
```console
$ deactivate
```

### Attribution
Parts of this [Contributing](#contributing) section were based on this [contributing.md guide](https://contributing.md/)
