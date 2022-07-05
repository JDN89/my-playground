<script setup lang="ts">
// const name = $ref('')

const router = useRouter()
const go = () => {
  router.push('/test')
}
</script>

<template>
  <div>
    <button class="border-light-50 border-2 p-2" @click="go">
      click Me
    </button>
  </div>
</template>
