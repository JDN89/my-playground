<script setup lang="ts">
// const name = $ref('')

const router = useRouter()
let message = $ref('')
const postMessage = $ref('')
let deleteMessage = $ref('')
const go = () => {
  router.push('/test')
}

const getData = () => {
  fetch('https://localhost:7096/api/test').then(res => res.text()).then(t => message = t)
}
const deleteData = () => {
  fetch('https://localhost:7096/api/test', { method: 'Delete' }).then(res => res.text()).then(t => deleteMessage = t).catch(Error => console.error(Error))
}
const postData = (postMessage: string) => {
  fetch('https://localhost:7096/api/test', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title: postMessage }),
  }).then(res => console.log(res.status))
}
</script>

<template>
  <div class="flex mx-auto flex-col flex-wrap ">
    <button class=" border-light-50 border-2 p-2 flex-wrap" @click="go">
      click Me
    </button>
    <button @click="getData">
      Fetch data from api
    </button>
    <button @click="deleteData">
      Delete me
    </button>
    <h2 v-if="message" class="border-1 border-light-50">
      {{ message }}
    </h2>
    <h2 v-if="deleteMessage" class="border-1 border-light-50">
      {{ deleteMessage }}
    </h2>
    <input v-model="postMessage" type="text" @keyup.enter="postData(postMessage)">
    <h2>
      {{ postMessage }}
    </h2>
  </div>
</template>
