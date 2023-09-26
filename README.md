ebase - emoji coding (like `base64` with emojis)
================================================

**This project meant to be only for educational purposes, so under no circumstances should it be used for production purposes.**  
However, contributions are very much welcome. Perhaps we could reach a stable version.

Unlike other base64-like emoji libs on the internet, this one is not using external libs or emoji metadata.

> **Important**  
> the encoding signature most probably **WILL** change!

### Parts
The package contains a `lib` and a `bin` too.

The binary meant to be a drop-in "replacement" for `base64`.

### Using the `lib`
todo

### Using the `bin`
```bash
$ echo "hello" | ebase
# => 💈💅💌💌💏

$ echo "💈💅💌💌💏" | ebase -d
# => hello

$ cat .gitignore | ebase
# => 🎏📄💁📂💇💅📄🎏🐃💁📂💇💏🎎💌💏💃💋🎏🎎💉💄💅💁

```

### Known bugs and limitations
 - There is some collision error with one character emojis
 - Line endings are not handled
 - Binaries are a mess

### Future plans
 - Investigate to split bytes differently
 - Investigate to use 2-byte emojis
 - Compress output