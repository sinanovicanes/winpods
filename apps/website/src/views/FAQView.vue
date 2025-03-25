<template>
  <main class="container grid grid-cols-1 gap-4 pt-5">
    <h1 class="text-3xl font-semibold">Frequently Asked Questions</h1>
    <div class="collapse collapse-arrow bg-base-100 border border-base-300">
      <input type="radio" name="faq" :checked="true" />
      <div class="collapse-title font-semibold">How do I create an account?</div>
      <div class="collapse-content text-sm">
        Click the "Sign Up" button in the top right corner and follow the registration process.
      </div>
    </div>
    <div class="collapse collapse-arrow bg-base-100 border border-base-300">
      <input type="radio" name="faq" />
      <div class="collapse-title font-semibold">I forgot my password. What should I do?</div>
      <div class="collapse-content text-sm">
        Click on "Forgot Password" on the login page and follow the instructions sent to your email.
      </div>
    </div>
    <div class="collapse collapse-arrow bg-base-100 border border-base-300">
      <input type="radio" name="faq" />
      <div class="collapse-title font-semibold">How do I update my profile information?</div>
      <div class="collapse-content text-sm">
        Go to "My Account" settings and select "Edit Profile" to make changes.
      </div>
    </div>
  </main>
</template>
