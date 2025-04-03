<script setup lang="ts">
const faq = [
  {
    question: 'What is winpods?',
    answer:
      'winpods is a lightweight desktop application that provides AirPods integration for Windows. It allows users to monitor battery levels, track connection status, and control AirPods directly from their Windows desktop.',
  },
  {
    question: 'Is winpods free?',
    answer:
      'Yes! winpods is an open-source project, meaning it’s completely free to use and modify.',
  },
  {
    question: 'Which AirPods models are supported?',
    answer:
      'winpods supports all AirPods models, including AirPods 1st and 2nd generation, AirPods Pro, and AirPods Max.',
  },
  {
    question: 'Which versions of Windows are supported?',
    answer:
      'winpods is designed to work on Windows 10 and Windows 11. Older versions might work but are not officially supported.',
  },
  {
    question: 'Does winpods automatically connect my AirPods to Windows?',
    answer:
      'No, winpods does not handle Bluetooth pairing. You need to connect your AirPods manually via Windows Bluetooth settings, and winpods will monitor their status once connected.',
  },
  {
    question: 'Does winpods detect when I remove an AirPod?',
    answer:
      'Yes! winpods automatically detects when an AirPod is removed and can send notifications or adjust audio settings accordingly.',
  },
  {
    question: 'Can I disable low battery notifications?',
    answer: 'Yes, you can customize notifications in the settings menu.',
  },
  {
    question: 'How do I install winpods?',
    answer:
      'Download the latest version from the official repository or release page, run the installer, and follow the on-screen instructions.',
  },
  {
    question: 'winpods is not detecting my AirPods. What should I do?',
    answer:
      'Ensure your AirPods are paired and connected to Windows. Restart winpods, check if Bluetooth is enabled, and try re-pairing your AirPods if necessary.',
  },
  {
    question: 'How do I update winpods?',
    answer:
      'winpods will notify you of available updates. You can also check for updates manually in the settings menu or download the latest version from the official repository.',
  },
  {
    question: 'How do I report a bug or request a feature?',
    answer:
      'You can report bugs or request features on the official GitHub repository under the "Issues" section.',
  },
  {
    question: 'How do I uninstall winpods?',
    answer: 'Go to "Settings > Apps > winpods" and click "Uninstall".',
  },
  {
    question: 'How can I contribute to winpods?',
    answer:
      'Contributions are welcome! You can report bugs, suggest features, submit pull requests, or help with translations and documentation via GitHub.',
  },
  {
    question: 'Will winpods support other Bluetooth earbuds in the future?',
    answer:
      'Currently, winpods is optimized for AirPods, but support for other Bluetooth earbuds may be considered in future updates.',
  },
];
</script>

<template>
  <main class="container grid grid-cols-1 gap-4">
    <h1 class="text-3xl font-semibold">Frequently Asked Questions</h1>
    <div
      v-for="(q, i) in faq"
      :key="i"
      class="collapse collapse-arrow bg-base-200 border border-base-300"
    >
      <input type="radio" name="faq" :checked="i === 0" />
      <div class="collapse-title font-semibold text-lg">{{ q.question }}</div>
      <div class="collapse-content text-sm">
        {{ q.answer }}
      </div>
    </div>
  </main>
</template>
