## Hydra
Hydra is a configuration management platform written in Rust. It consists of two parts, a control application and a small service agent. After creating configuration files specific to your network environment, you will interact with the control application using it's REPL. The distributed agent binary is used to collect information and perform configuration tasks on each of your servers and devices, it is used by the control application and is not executed directly by the user. 

#### Why?
Many of the existing systems created to manage platform with code seem trapped in their own domain specific abstractions. Configuration Mangement syntax should be as terse as possible while still allowing the required flexibility to describe complex networks and systems. Also, Rust is fast, safe and has great concurrency tools which should help it overcome(avoid?) performance and scaling problems faced by other tools with similar goals.

#### Warning/Disclaimer
This project is very young and will have continuous breaking changes for the unforseen future. Unless you are working on the project and tracking the changes you'll probably have a bad time trying to "use it" in a predictable manner.

#### Core Concepts
- Hydra refers to the hosts and devices it manages as __inventory__
- Groups of services and interdependent applications and functionality are described as __roles__ 
- Presently, __all__ communication happens over SSH
- The main user interface is a REPL/Command shell
- Configuration is queried using the REPL
- Node and service configuration is done with JSON files for safe versioning.
- Ad-hoc configuration and role application is done using the REPL

The agent should be deployed on every OS/host/container/device in your network and/or cluster that you would like to manage with Hydra. You should have ssh access to every machine/instance you are managing. Presently this needs to be key-based, Hydra does not support password authentication.

The Linux/MacOS server/device agent can be found [here](https://github.com/alanwilhelm/hydra-agent).

#### Control Node Requirements
- Use `rustup default nightly`
- RocksDB

#### Agent Requirements
- Use `rustup default nightly`
- OpenSSL `sudo apt-get install pkg-config libssl-dev`
- Valid home directory for SSH user/service owner and proper permissions to write to ~/.hydra/


## Examples

```
> ping
Pinging known hosts:
node 111.222.333.444 up
node 222.333.444.555 up
node 333.444.555.666 up
```

This information should be stored to RockDB
```
> profile_hosts
Getting remote host system profiles:
SystemProfile { cpu_num: "16", cpu_speed: "2199", hostname: "dev-1", os_type: "Linux", os_release: "4.4.0-53-generic" }
SystemProfile { cpu_num: "12", cpu_speed: "2596", hostname: "dev-2", os_type: "Linux", os_release: "4.4.0-87-generic" }
SystemProfile { cpu_num: "8", cpu_speed: "2397", hostname: "dev-3", os_type: "Linux", os_release: "4.4.0-92-generic" }
```

```
> host lookup dev-1
Looking up id for host: dev-1
ecb1b6d2-9fc6-4ed3-8558-c9c52a57f8b7
```

```
> host ecb1b6d2-9fc6-4ed3-8558-c9c52a57f8b7 deploy [role]  
```


## Features Todo:
- [x] Return basic Linux/Unix system stats
- [ ] Return advanced Linux/Unix system stats
- [x] Docker daemon inspection
- [ ] Docker daemon process/container control
- [ ] Query, start and stop Linux systemd services
- [ ] Postgres control and stats
- [ ] NodeJS monitor code redeploy
- [ ] Erlang/Elixir/OTP application management
- [ ] Blockchain server control/management via JSON-RPC(parity/geth, monerod, bitcoind)
- [ ] Easily deploy cross-compiled agent to servers in inventory
- [ ] Shell script library

## Dev Todo
- [x] Return Cpu count, CPU speed, OS, Release
- [x] Command line REPL
- [ ] Return disk space stats
- [ ] Return memory usage stats
- [ ] Struct for Error Response
- [ ] Improve IO management
- [ ] Store returned SystemProfile(s) to RocksDB
- [ ] Remove experimental dead code
- [ ] Write some real tests

## Tests
`$ cargo test`

## Ideas and additional features
- Describe, deploy and maintain infrastructure and operations with code and configuration
- Manage complex med-large scale clusters and networks using an inventory configuration model
- Operate across geographically distributed and hybrid networks 
- Be secure and do not require additional network configuration.
- Assume the individual servers may be behind an ssh jumpbox and aid in configuration
- Install and manage applications and dependent services in groups with "roles" model
- Address major package management systems such as Apt and Yum
- Add ability to configure downloading and building binaries from source
- Split concerns between controller and server agent
- Directly address useful services such as Docker & Postgres
- Keep the server agent small and fast as possible.
- Collect host and tenant performance stats across networks/clusters
- Monitor and control various application specific services
- Run commands, scripts, native hydra_agent processes across groups of hosts/VMs
- Query, start and stop services
- Deploy cross-compiled agent to server inventory using only ssh access
- Manage selected services on AWS and Azure
# hydra
