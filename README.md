# Grapher

**Grapher** (a.k.a. `graph`) is a small command-line tool for encoding and decoding strings in several formats, very useful in cybersecurity, CTFs, pentesting or everyday development.


## Installation

### 1. Clone the repository

```bash
git clone https://github.com/CodingLikeABaby/grapher.git
cd grapher
```

### 2. Add it to Path using cargo

```bash
cargo build --release
cp target/release/graph ~/.cargo/bin/
```

If you not already done , you'll need to install cargo

```bash
curl https://sh.rustup.rs -sSf | sh
```
When done, you should see somehting like this

```bash
Rust is installed now. Great!
```


### 3. Have fun !


## How to use it ?

(Here is some short flags examples, you can use the `--help` flag to see te long ones.

Grapher currently offers 2 different encodings, Base64 and URL encoding.

Grapher use Uppercase flags to encode and lowercase to decode.

```bash
grapher -B hello      # Base64 encode
grapher -b aGVsbG8=   # base64 decode

grapher -U "hello bro"      # URL encode
grapher -u hello%20bro      # URL decode
```

If you want to use spaces in your strings, you must use quotes : "my string"

You can also use double encoding with the `-d` flag.

```bash
graph -B -d hello      # Duble Base64 encode
graph -b -d YUdWc2JHOD0=  # Double Base64 decode
graph -U -d "hello bro"     # Duble URL encode
graph -u -d hello%2520bro  # Double URL decode
```

You can use Base64 URL encode too, using the `-M` mix flag.

```bash
graph -M hello      # Base64 URL encode
graph -m aGVsbG8%3D # Base64 URL decode
```

### Important

Grapher has several invalid combinations that will return an error with an alternative command if you trigger them.
This is because Grapher encourages you to use it as simply as possible.

However, to be more efficient, we've added a copy function to the Clipboard. You can use it with the `-c` flag.

```bash
grapher -B -c hello      # This will copy the output to the clipboard (aGVsbG8=)
```



### About Grapher

Grapher is written in Rust using multiple librairies to handle the clipboard functions and of course, encoding.

Made by @CodingLikeABaby 













