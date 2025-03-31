<script setup lang="ts">
const faq = [
  {
    question: 'How do I create an account?',
    answer:
      'Click the "Sign Up" button in the top right corner and follow the registration process.',
  },
  {
    question: 'I forgot my password. What should I do?',
    answer:
      'Click on "Forgot Password" on the login page and follow the instructions sent to your email.',
  },
  {
    question: 'How do I update my profile information?',
    answer: 'Go to "My Account" settings and select "Edit Profile" to make changes.',
  },
];
</script>

<template>
  <main class="container grid grid-cols-1 gap-4 pt-5">
    <h1 class="text-3xl font-semibold">Frequently Asked Questions</h1>
    <div
      v-for="(q, i) in faq"
      :key="i"
      class="collapse collapse-arrow bg-base-100 border border-base-300"
    >
      <input type="radio" name="faq" :checked="i === 0" />
      <div class="collapse-title font-semibold text-lg">{{ q.question }}</div>
      <div class="collapse-content text-sm">
        {{ q.answer }}
      </div>
    </div>
  </main>
</template>
