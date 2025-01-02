<p align="center">
 <img src="static/onions.svg">
</p>



<p align="center">
 <img alt="GitHub Last Commit" src="https://img.shields.io/github/last-commit/tuxicorn/onions.watch" />
 <img alt="PR friendly" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat" />
 <img alt="Python Version" src="https://img.shields.io/github/pipenv/locked/python-version/tuxicorn/onions.watch" />
 <img alt="Flask Version" src="https://img.shields.io/github/pipenv/locked/dependency-version/tuxicorn/onions.watch/flask/master" />
 <img alt="Requests Version" src="https://img.shields.io/github/pipenv/locked/dependency-version/tuxicorn/onions.watch/requests/master" />
 
</p>
</br>


</br>

This project is a copy of [dark.fail](https://dark.fail) and aims to provide a codebase to allow anyone to create their own website that assess the uptime of popular [onion services](https://community.torproject.org/onion-services/).
</p>


</br>
</br>

### Install Tor

Debian or Ubuntu

```sh
sudo apt install tor -y
```

RHEL, CentOS or Fedora

```sh
sudo dnf install tor -y
```

Archlinux and other arch-based distribution

```sh
sudo pacman -S tor
```

### Install pipenv and create a virtual environement

```sh
pip install pipenv
```
Create an active environemnt

```sh
pipenv shell
```
Install packages into the pipenv virtual environment

```sh
pipenv install
```

### Run the app
  
```sh
pipenv run python app.py 
```

</br>
</br>
</br>
