# cse2421-lab-helper
Helps auto-generate C files, updates Makefile, and updates header.

How to use:

run:
```console
cse2421 new {param}
```

There are currently only 2 options:

```console
cse2421 new project
```
which takes you step-by-step to create a new lab project in your current directory

```console
cse2421 new file
```
Which allows you to add a new function file. This command automatically updates the header file and Makefile. 
Note: Make sure to run this file from within the project directory

How to Build:

Once you have cloned the repo, change directory and run:
```console
cargo build --release
```

The executable should then be under /target/release/cse2421-lab-helper

I will also upload a pre-build executable for stdlinux.

## For OSU Students:
Follow these instructions to use this tool

1. Download pre-built executable for stdlinux
2. Transfer the file from your computer to your stdlinux enviroment. If you are on a UNIX machine you can use the "scp" command. I don't use Windows so I don't know.
3. Run the following commands 
```console
mkdir bin
mv cse2421-lab-helper ~/bin/cse2421-lab-helper
```
4. Add the following to your .bashrc file using:
```console
vi ~/.bashrc
```
Add: export PATH="$HOME/bin:$PATH"
Save the file and then type the command:
```console
source ~/.bashrc
```

Now you should be all set up go to.
Just note: this is a very basic program and has little to no error handling so make sure you type in your inputs correctly.
