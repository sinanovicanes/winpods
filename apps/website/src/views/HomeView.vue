<script setup lang="ts">
import WPButton from '@/components/WPButton.vue';
</script>

<template>
  <main class="flex flex-col items-center justify-center text-white">
    <section class="flex flex-col justify-center items-center gap-6 p-6 md:p-10">
      <div class="flex flex-col justify-center items-start">
        <h1 class="text-3xl font-bold mb-4">Enhance your AirPods experience on Windows</h1>
        <p class="text-gray-300 leading-relaxed mb-6">
          Lorem ipsum dolor, sit amet consectetur adipisicing elit. Asperiores quam at a sit
          quibusdam ducimus nulla maiores quasi perspiciatis placeat, unde, officia beatae quos,
          dolorum cumque rerum. Neque, perspiciatis. Porro, ipsam maiores accusamus blanditiis
          placeat rem nesciunt quas consequuntur ipsa nam! Ipsa ullam natus cumque iusto voluptas
          voluptatum dolorum, dolorem suscipit, libero sint culpa assumenda velit ipsam iste
          consequuntur provident obcaecati facilis! Sint veniam, quisquam deleniti asperiores vel
          ratione fugit totam incidunt atque fuga quas facere. Quaerat architecto iure temporibus
          quam vel maxime libero rerum odio numquam! Dolore, animi voluptates similique consectetur
          sunt sint. Aliquid saepe fuga labore praesentium quae.
        </p>
      </div>
    </section>
    <section class="container mx-auto grid md:grid-cols-2 gap-10 p-6 md:p-10 items-center">
      <div class="flex justify-center items-center">
        <img
          src="@/assets/widget.png"
          alt="AirPods Windows App"
          class="object-cover rounded-md mx-auto"
        />
      </div>
      <div class="flex justify-center items-center">
        <img src="@/assets/app.png" alt="AirPods Windows App" class="object-cover w-full" />
      </div>
    </section>
  </main>
</template>
