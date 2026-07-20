import type { VercelRequest, VercelResponse } from '@vercel/node';

const QUERY = `
query SearchForumComments(
  $owner: String!,
  $repo: String!,
  $number: Int!
) {
  repository(owner: $owner, name: $repo) {
    discussion(number: $number) {
      comments(first: 100) {
        nodes {
          id
          body 
          url
          author {
            login
          }
        }
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

  try {
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
          number: 140,
        },
      }),
    });

    if (!response.ok) {
      return res.status(response.status).json({
        error: 'GitHub GraphQL request failed',
      });
    }

    const json = await response.json();

    if (json.errors) {
      return res.status(500).json(json.errors);
    }

    const comments =
      json.data?.repository?.discussion?.comments?.nodes ?? [];

    const result = comments
      .filter((comment: { body: string }) =>
        comment.body.toLowerCase().includes(q)
      )
      .map(
        (comment: {
          id: string;
          body: string;
          url: string;
          author?: { login: string };
        }) => ({
          id: comment.id,
          author: comment.author?.login ?? 'Unknown',
          preview:
            comment.body.length > 180
              ? `${comment.body.slice(0, 180)}...`
              : comment.body,
          url: comment.url,
        })
      );

    return res.status(200).json(result);
  } catch {
    return res.status(500).json({
      error: 'Internal Server Error',
    });
  }
}