import { GithubAPI } from '@winpods/github-api';

export class DownloadService {
  static async startDownload() {
    try {
      const latestRelease = await GithubAPI.getLatestRelease(import.meta.env.VITE_GITHUB_REPO);
      const asset = latestRelease.assets.find((asset) => asset.name.endsWith('.exe'));

      if (!asset) {
        throw new Error('No asset found');
      }

      const downloadUrl = asset.browser_download_url;
      const downloadName = asset.name;

      const a = document.createElement('a');
      a.href = downloadUrl;
      a.download = downloadName;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
    } catch (e) {
      console.error(e);
      alert('Failed to start download');
    }
  }
}
