import yargs from 'yargs';
import { createAnkiCsvImportFile } from './ankiCsv';

import { fetchEnglishProfile } from './english-profile';
import { CEFR_LEVELS } from './models';

async function execute() {
  const argv = await yargs(process.argv)
    .option('filterLevel', {
      alias: 'l',
      description: 'Filter level',
      array: true,
      type: 'string',
      choices: Object.keys(CEFR_LEVELS),
    })
    .option('outputFile', {
      alias: 'o',
      description: 'Output Filename',
      type: 'string',
      default: 'ankiCollection',
    })
    .parseAsync();

  const wordsDetails = await fetchEnglishProfile(argv.filterLevel);

  await createAnkiCsvImportFile(wordsDetails, argv.outputFile);
}

execute()
  .then();
