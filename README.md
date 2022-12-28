# Anki English Generator

The console app to generate the Anki cards based on [englishprofile.org](https://www.englishprofile.org/wordlists/evp) word list.

## Usage

```console
cargo build

cargo run -- --help
cargo run -- --filter-level a1 --filter-topic technology --output-file ./out/result
```

## Debugging

Debugging is configured with Visual Studio Code.

## ToDo

- [x] generate and export into CSV file
- [ ] add American english generation (requires to investigate the issue with no ID in URL)
- [ ] add apkg support
- [ ] include audio pronunciation support
- [ ] review and improve the Anki template
