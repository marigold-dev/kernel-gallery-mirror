# Tzwitter

The project is divided into two sub projects:

- the kernel, that is ran by the rollup
- the front, that can be uploaded anywhere

This readme describe how to deploy the twitter clone with a node running on your computer listenning on port 18731

## The kernel

The kernel is developped with Rust and compiled to Wasm.

You will need rust 1.66 and the wasm32-unknown-unknown target installed

```bash
$ export TZWITTER_L1_CONTRACT=kt1 # Should replace this with real contract address (when running on a smart rollup (see deploy script))
$ cargo build --manifest-path kernel/Cargo.toml --release --target wasm32-unknown-unknown
```

Architecture:

The kernel has several stages:

- read the inbox
- check signatures
- applying messages

TODO: benchmark each stages and check if they each take less than 11_000_000_000 ticks to be executed. Otherwise we will have to change this (opiniated) architecture to the following one:

- read one message, check it signature, applying it
- do it n times (where n is represent n iteration to reach ~11_000_000_000 ticks)
- save the inbox and reboot

The first solution is _easier_ to implement in my opinion and easier to read for a developper. Remember that this project is an example to show how to develop a kernel

## How to compile

```bash
$ cargo build --release --manifest-path kernel/Cargo.toml --target wasm32-unknown-unknown
```

## How to deploy

Because the kernel is bigger than 24kb, there is a deploy script to help you to install your kernel

```bash
./deploy.sh account_alias
```

> Your account alias will need 10k tez to post commitment.
> The deploy script was designed to work on ProtoAlpha

Then execute the last command shown by this script

## How to start the React Application

- Change your current working directory to the newly cloned repository directory.

    `cd app`

- Install dependencies:

   `npm install`

- Start development server:

   `npm run dev`

- Open http://localhost:5173/ in your browser to see a sample application.

## Building the App

- Run the command:

   `npm run build`

- After you have build the app, you can run it locally:

   `npm run preview`

- Open http://localhost:4173/ in your browser to see the production build. For more information read the [Vite Guide](https://vitejs.dev/guide/static-deploy.html)
