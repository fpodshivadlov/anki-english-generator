use std::fs;
use std::path::Path;

pub fn with_cache(cache_key: &str, func: fn() -> &str) -> &str {
  // ToDo: implement cache
  let result = func();
  result
}

// export async function fetchHtmlCached(url: string, fetchOptions: AxiosRequestConfig = {}): Promise<string> {
//   const cacheDirPath = 'cache';
//   const cacheSuffix = fetchOptions.data ? `_${pathify(fetchOptions.data.toString())}` : ``;
//   const cachePath = path.join(cacheDirPath, `_${pathify(url)}${cacheSuffix}.html`);
//   if (fs.existsSync(cachePath)) {
//     console.debug(` page extracted from cache: ${url}`);
//     return fs.readFileSync(cachePath, 'utf8');
//   } else {
//     fs.mkdirSync(cacheDirPath, { recursive: true });
//     const html = await fetchHtml(url, fetchOptions);
//     await new Promise((resolve) => setTimeout(resolve, FETCH_DELAY));
//     fs.writeFileSync(cachePath, html);
//     console.debug(` page cached: ${url}`);
//     return html;
//   }
// }
