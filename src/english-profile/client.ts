import { CEFR_LEVELS } from '../models';
import { fetchHtmlCached } from './httpUtils';

export async function fetchWordsHtml(levelFilter?: string[]) {
  let formData = new URLSearchParams();
  formData.append('limit', '0');
  if (levelFilter) {
    levelFilter.forEach(levelValue => {
      formData.append('filter_custom_Level[]', `${CEFR_LEVELS[levelValue]}`);
    });
  }

  let body = formData.toString();
  const html = await fetchHtmlCached(
    'https://www.englishprofile.org/wordlists/evp',
    {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
      },
      data: body,
    }
  );
  return html;
}

export async function fetchWordDetails(detailsUrl: string) {
  const html = await fetchHtmlCached(
    detailsUrl,
    {
      method: 'GET',
    }
  );
  return html;
}