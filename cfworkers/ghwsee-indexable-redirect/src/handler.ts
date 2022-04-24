interface OriginalInfo {
  indexable: boolean
}

export async function handleRequest(request: Request): Promise<Response> {
  const githubUrl = new URL(
    request.url.replace('github-wiki-see.page/m', 'github.com'),
  )

  const ghwseeResponse = fetch(request, {
    cf: {
      cacheEverything: true,
      cacheTtl: 7200,
    },
  })

  const pathComponents = githubUrl.pathname.split('/')
  if (pathComponents.length > 3 && pathComponents[2] === 'wiki_index') {
    return await ghwseeResponse
  }

  console.log(request.headers.get('user-agent'))

  try {
    const info = await originalInfo(githubUrl)
    if (info.indexable) {
      console.log('Indexable Redirect: ' + githubUrl.href)
      return new Response(null, {
        status: 308,
        statusText: 'Permanent Redirect',
        headers: {
          Location: githubUrl.toString(),
        },
      })
    }
  } catch (e) {
    console.error(e)
  }

  console.log('No Redirect: ' + githubUrl.href)

  const response = await ghwseeResponse
  if (response.status === 308 && !request.url.endsWith('/wiki/Home')) {
    console.warn('Redirected Unindexable: ' + response.headers.get('Location'))
  }

  return await ghwseeResponse
}

export async function originalInfo(url: URL): Promise<OriginalInfo> {
  const response = await fetch(url.toString(), {
    redirect: 'follow',
    cf: {
      cacheEverything: true,
      cacheTtl: 86400,
    },
  })
  if (response.status != 200) {
    return {
      indexable: false,
    }
  }
  if (response.headers.has('x-robots-tag')) {
    return {
      indexable: false,
    }
  }
  return {
    indexable: true,
  }
}
