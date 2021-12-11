# Anki English Generator

The console app to generate the Anki cards based on [englishprofile.org](https://www.englishprofile.org/wordlists/evp) word list.

## Usage

```console
npm install

npm start -- --help
npm start -- --filterLevel A1 --filterTopic technology --outputFile ./out/result
```

## Debugging

Debugging is configured with Visual Studio Code.

## ToDo

- [x] generate and export into CSV file
- [ ] add American english generation (requires to investigate the issue with no ID in URL)
- [ ] add apkg support
- [ ] include audio pronunciation support
- [ ] review and improve the Anki template
