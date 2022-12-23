// @ts-check

import fs from 'fs/promises'
import { parseMarkdown } from '@yandeu/parse-markdown'

const main = async () => {
  const indexFilePath = './src/files/index.html'

  const markdownInput = await fs.readFile('README.md', { encoding: 'utf-8' })
  const indexFile = await fs.readFile(indexFilePath, { encoding: 'utf-8' })

  const { markdown } = await parseMarkdown(markdownInput)

  let html = indexFile.replace('{MARKDOWN}', markdown)

  await fs.writeFile(indexFilePath, html, { encoding: 'utf-8' })
}
main()
