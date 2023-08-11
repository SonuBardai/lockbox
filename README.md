# Lockbox
<img src="https://i.imgur.com/ZV550bh.jpg" alt="Lockbox" width="100%">


Lockbox is a command-line tool for generating and managing passwords. It uses strong encryption algorithms to securely store your passwords, so you can be sure that your data is safe.

[![codecov](https://codecov.io/gh/SonuBardai/lockbox/branch/main/graph/badge.svg?token=XV39653NA5)](https://codecov.io/gh/SonuBardai/lockbox)

### Usage
To use Rust Password Manager, first make sure you have Rust installed on your system. Then, clone this repository and run cargo build to build the project.

Once you’ve built the project, you can run it using cargo run. Here’s an overview of the available commands:

```rust
                                               
                                                           ..7J?..   ..^JJ7..
                                                       :~~JPG5PB5PG ~#PPBBGGG5~~:
                                                       ?&J?JPPY7:J& !@7~?##5??5&BJ
                                                   :!PP5~JPB#57#GY ^PYJ7G#B77Y5GB!:
                                                   7@BP555PPG#&Y^     ^JPBBP5555P@7
                                                   :?##BGGGG7^^         ^~~5GGBBB?^
                                                   7&P5P&?    .. .......    7@BPP&7
                                                   .^5PGGG5~5PY5BB55GBG5G5~5PPBG5^.
                                                       ~J!5BGJ77JGP??5GP7YPGBBBY~
                                               ^5BPPPPY5P5YY55J!7775Y77?5PGGPPPPBP~.
                                               .^YG5JPGB#@#BBGP5555555555PGGB#@#BGGYY@G?.
                                               7&Y:!?5&@&BB#@BBBPPPPPPPPPGB#&&B&@&5?75J5&:
                                               7@^ !JPBBGGGGGBGGBBBBBBBBBB##GPGGBB5#P  !#:
                                               7@^ GB5JP#:................... .BGJ5#P   .
                                               :!. P#B7JP5!.                .~5PJ7B#Y
                                                   7@.!GPGP?:            :7PGPG7.@?
                                                   :~ :!PGG&~            !&BGG7: ~:
                                                       ....              ....
                                               
                                               L🦀CKBOX: A password manager and generator

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
Contributions are welcome! If you’d like to contribute, please feel free to open an issue or submit a pull request. Checkout our [CONTRIBUTING](CONTRIBUTING.md) file for details on how to contribute.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
