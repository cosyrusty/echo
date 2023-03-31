# echo
echo in rust.
Display a line of text.

## usage
  ```
  $ cargo run -- hello
  hello
  ```

  ```
  $ cargo run -- -n hello
  $ cargo run -- -e 'hello\nworld'
  ```

## following sequences are interpreted when option -e is set

\\      backslash
\a      alert (BEL)
\b      backspace
\c      produce no further output
\e      escape
\f      form feed
\n      new line
\r      carriage return
\t      horizontal tab
\v      vertical tab

TODO

\0NNN   byte with octal value NNN (1 to 3 digits) 
\xHH    byte with hexadecimal value HH (1 to 2 digits)

## sudo code
  * read cmdline line arguments
  * parse the options if any in args
  * otherwise store the args in vec
  * print each string in args according to parsed option
