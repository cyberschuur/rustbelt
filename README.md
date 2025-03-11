# Rustbelt

<div align="center">

<img src="assets/rustacean-belted.png" width=400px>

</div>

[Rustbelt](https://en.wikipedia.org/wiki/Rust_Belt) is a Rust implementation of the Windows enumeration tool [Seatbelt](https://github.com/GhostPack/Seatbelt). The purpose of this project is to provide a fast, efficient, and safe alternative to Seatbelt, leveraging the powerful features of the Rust programming language. This project was created as a learning exercise to enhance my understanding of Rust and how to use it with the native Windows API.

> Please note that, in it's current form, there are some features missing which I am planning to add soon. Regardless of missing modules, the most urgent additions are the following:
> 
> - **Support for remote execution** (): Even though username and password is handled appropriately when using WMI queries, there is not yet any support for using them on remotely managed machines. The option `computer_name` currently does not do anything for that reason.
> - **Proper handling of output formats** (): I've added abstractions in the `src/runtime/formatter` and `src/runtime/writer` modules, but these still need to be used in the `Runtime` object. Currently, I've just called them from the main function. Very lazy, but hey, feel free to fix this!
> - **Completing registry interface** (): The registry util is only partially implementated compared to Seatbelt. This means that operations performed in certain Seatbelt modules might not exist yet. Furthermore, these should probably be accessible from, or take the Runtime as an argument; again, for remote execution.
> - **Improved logging and error handling** (): Logging is very minimal (or actually non-existent?) and as I am a Rust n00b, the error handling could most definitely use some work.

## Purpose

The primary goal of Rustbelt is to enumerate various security-related settings and configurations on Windows systems. It aims to provide detailed information about the system's security posture, similar to what Seatbelt does, but with the added benefits of Rust's memory safety and concurrency features.

## Contributing

This project was created as a learning exercise, and I welcome contributions from anyone interested in improving Rustbelt. Whether you want to add new features, fix bugs, improve documentation, or optimize existing code, your contributions are highly appreciated.

As it currently stands, there are not many modules. Adding a new module should be relatively easy as you can use the existing Seatbelt implementation, and I have provided examples and utils for WMI and the working with the windows registry. Please not that the latter is very incomplete and you might need to 

Feel free to fork the repository, make your changes, and submit a pull request. 

## License

Rustbelt is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
