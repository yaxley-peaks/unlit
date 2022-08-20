### Literate Programming for all!

Haskell style literate programming for all languages

> Literate programming is a programming
> paradigm introduced by Donald Knuth in
> which a computer program is given an
> explanation of its logic in a natural
> language, such as English, interspersed
> (embedded) with snippets of macros and
> traditional source code, from which
> compilable source code can be
> generated. The approach is used in
> scientific computing and in data
> science routinely for reproducible
> research and open access purposes.
> Literate programming tools are used by
> millions of programmers today.


#### Installation
1. Install the rust compiler and cargo.
2. Clone this repository.
3. Enter the directory. `cd unlit`
4. Run: `cargo install --path .`

#### Usage
```
USAGE:
    unlit [OPTIONS] -i <IN_FILE>

OPTIONS:
    -c <COM>             Single line comment character sequence. Default: '//'
    -h, --help           Print help information
    -i <IN_FILE>         Input file
    -o <OUT_FILE>        Output file. Default: <stdout>
    -V, --version        Print version information
```