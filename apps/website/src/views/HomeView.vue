<script setup lang="ts"></script>

<template>
  <main class="flex flex-col gap-4">
    <h1 class="text-3xl font-semibold">Enhance your AirPods experience on Windows</h1>
    <section class="hero bg-base-200">
      <div class="hero-content flex-col lg:flex-row">
        <img
          src="@/assets/widget.png"
          alt="AirPods Windows App"
          class="max-w-sm rounded-lg shadow-2xl"
        />
        <div>
          <h1 class="text-5xl font-bold">Widget View</h1>
          <p class="py-6">
            Quickly access your AirPods settings and battery levels with the widget view.
          </p>
        </div>
      </div>
    </section>
    <section class="hero bg-base-200">
      <div class="hero-content flex-col lg:flex-row">
        <div>
          <h1 class="text-5xl font-bold">Dashboard</h1>
          <p class="py-6">
            Manage your AirPods settings, battery levels, and more with the dashboard view.
          </p>
        </div>
        <img
          src="@/assets/app.png"
          alt="AirPods Windows App"
          class="max-w-sm rounded-lg shadow-2xl"
        />
      </div>
    </section>
    <section class="hero bg-base-200">
      <div class="hero-content flex-col lg:flex-row">
        <div>
          <h1 class="text-5xl font-bold">Notifications</h1>
          <p class="py-6">Receive notifications for battery levels, connection status, and more.</p>
        </div>
      </div>
    </section>
  </main>
</template>
