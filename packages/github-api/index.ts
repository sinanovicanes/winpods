export interface AssetInfo {
  name: string;
  browser_download_url: string;
}

export interface ReleaseInfo {
  tag_name: string;
  body: string;
  published_at: string;
  assets: AssetInfo[];
}

export class GithubAPI {
  static async getLatestRelease(repo: string): Promise<ReleaseInfo> {
    const response = await fetch(`https://api.github.com/repos/${repo}/releases/latest`, {
      method: "GET",
      headers: {
        "User-Agent": "request",
        "Content-Type": "application/json"
      }
    });

    if (!response.ok) {
      throw new Error(
        `[GITHUB_API]: Failed to fetch latest release: ${repo} ${response.status} ${response.statusText}`
      );
    }

    return await response.json();
  }

  static async getReleaseByTag(repo: string, tag: string): Promise<ReleaseInfo> {
    const response = await fetch(
      `https://api.github.com/repos/${repo}/releases/tags/${tag}`,
      {
        method: "GET",
        headers: {
          "User-Agent": "request",
          "Content-Type": "application/json"
        }
      }
    );

    if (!response.ok) {
      throw new Error(
        `[GITHUB_API]: Failed to fetch release with tag: ${repo} ${tag} ${response.status} ${response.statusText}`
      );
    }

    return await response.json();
  }
}
