interface GithubAsset {
	name: string;
	browser_download_url: string;
}

interface GithubResponse {
	tag_name: string;
	body: string;
	published_at: string;
	assets: GithubAsset[];
}

export class GithubAPI {
	static async getLatestRelease(repo: string): Promise<GithubResponse> {
		const response = await fetch(`https://api.github.com/repos/${repo}/releases/latest`, {
			method: 'GET',
			headers: {
				'User-Agent': 'request',
				'Content-Type': 'application/json',
			},
		});

		if (!response.ok) {
			throw new Error(`[GITHUB_API]: Failed to fetch latest release: ${repo} ${response.status} ${response.statusText}`);
		}

		return await response.json();
	}
}
