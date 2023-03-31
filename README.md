# echo
echo in rust.

## usage
  ```
  $ cargo run -- hello
  hello
  ```

  ```
  $ cargo run -- -n hello
  $ cargo run -- -e 'hello\nworld'
  ```

## sudo code
  * read cmdline line arguments
  * parse the options if any in args
  * otherwise store the args in vec
  * print each string in args according to parsed option
