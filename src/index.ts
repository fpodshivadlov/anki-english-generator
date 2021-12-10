import yargs from 'yargs';

import { fetchEnglishProfile } from './english-profile';

async function execute() {
  const argv = await yargs(process.argv)
    .option('filterLevel', {
      alias: 'l',
      description: 'Filter level',
      array: true,
      type: 'string',
      choices: ['A1', 'A2', 'B1', 'B2', 'C1', 'C2']
    })
    .parseAsync();

  const wordsDetails = await fetchEnglishProfile(argv.filterLevel);
  console.log(wordsDetails.length);
}

execute()
  .then();
