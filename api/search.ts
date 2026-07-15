import type { VercelRequest, VercelResponse } from '@vercel/node';

const QUERY = `
query SearchDiscussions($owner: String!, $repo: String!) {
  repository(owner: $owner, name: $repo) {
    discussions(first: 100, orderBy: { field: UPDATED_AT, direction: DESC }) {
      nodes {
        title
        url
      }
    }
  }
}
`;

export default async function handler(
  req: VercelRequest,
  res: VercelResponse
) {
  const q = String(req.query.q ?? '').trim().toLowerCase();

  if (!q) {
    return res.status(200).json([]);
  }

  const response = await fetch('https://api.github.com/graphql', {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${process.env.GITHUB_TOKEN}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      query: QUERY,
      variables: {
        owner: 'soteenstudio',
        repo: 'lightvm',
      },
    }),
  });

  if (!response.ok) {
    return res.status(response.status).json({
      error: 'GitHub GraphQL request failed',
    });
  }

  const json = await response.json();

  const discussions =
    json.data?.repository?.discussions?.nodes ?? [];

  const result = discussions
    .filter((d: { title: string }) =>
      d.title.toLowerCase().includes(q)
    )
    .map((d: { title: string; url: string }) => ({
      title: d.title,
      url: d.url,
    }));

  res.status(200).json(result);
}