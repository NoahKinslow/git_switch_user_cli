This is a simple rust application that allows you to quickly change your active git user account in only two steps by leveraging git and github cli.

I often use multiple git accounts for different repos, I find it tedious to continously go through the process of changing the config and reauthenticating each time. This program automates the process by simply asking what username you'd like to use and authenticating you through the web browser using gh auth login.

showcase:

![showcase](https://user-images.githubusercontent.com/39603201/149581571-6e58eff0-b168-4902-87a0-cdfa96995583.png)


Requires that you have installed gh (github-cli).
This uses a --global change to the config so that you don't have to worry about individual projects.
I have only tested this for Github.com, not Github.com Enterprise Server. It is not intended to work with Enterprise Server.
