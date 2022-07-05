<script setup lang="ts">
const router = useRouter()

const back = () => router.push('/')
</script>

<template>
  <div>
    <h1 class="text-2xl">
      Hello
    </h1>
    <button class="border-2 dark:border-light-100 border-gray-700" @click="back">
      TAKE ME BACK
    </button>
  </div>
</template>
