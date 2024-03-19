First install Homebrew
~~~
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
~~~

Once Homebrew is installed you can install curl
~~~
brew install curl 
~~~

Verify installation 
~~~
curl --version 
~~~

If there is any issues during installation 
~~~
brew update
brew upgrade
~~~

For MacOS run "xcode-select --install" to get a C compiler

Then run:

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
