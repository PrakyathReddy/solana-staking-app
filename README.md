# solana-staking-app

**cargo.toml explanation**

[package] section is just some meta-information about your project, but none of that affects how your program will be identified once deployed, it’s just for the internal development process.

[dependencies] that are dependencies of your program, from now we only need a few packages:

- borsh & borsh-derive, those two handles necessary serialization for our data
- solana-program is a core dependency of any Solana program that provides all necessary APIs like entrypoint!, AccountInfo, etc.
- thiserror is not necessary, but it’s a popular package for error handling

[profile.release] this section configures how your program builds for production. We will not discuss most of the options, but overflow-checks = true is a tremendously important part that provides out-of-the-box safe math.

**Project Structure**

src/lib.rs - here we declare all our modules
src/entrypoint.rs - that’s an entry point of our program; this module requires very few code as there is not much logic related to the entry point
src/processor.rs - that the heart of our program, here we handle all of the logic; but for now it’s simple Ok(())
