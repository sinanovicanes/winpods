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
		console.log(`https://api.github.com/repos/${repo}/releases/latest`);
		const response = await fetch(`https://api.github.com/repos/${repo}/releases/latest`);

		if (!response.ok) {
			console.log(response);
			throw new Error('Failed to fetch latest release');
		}

		return await response.json();
	}
}
