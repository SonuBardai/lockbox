# Lockbox
<img src="https://i.imgur.com/ZV550bh.jpg" alt="Lockbox" width="100%">


Lockbox is a command-line tool for generating and managing passwords. It uses strong encryption algorithms to securely store your passwords, so you can be sure that your data is safe.

### Usage
To use Rust Password Manager, first make sure you have Rust installed on your system. Then, clone this repository and run cargo build to build the project.

Once you’ve built the project, you can run it using cargo run. Here’s an overview of the available commands:

```
A password manager and generator

Usage: lockbox <COMMAND>

Commands:
  add       
  generate  Generate a password with the specified properties [default: length=16, symbols=false, uppercase=true, lowercase=true, numbers=true, count=1]
  list      
  remove    
  show      
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
For example, to generate a new password with default properties, you can run cargo run -- generate.

### Working
[Store](./src/store/README.md)

### Contributing
Contributions are welcome! If you’d like to contribute, please feel free to open an issue or submit a pull request.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.