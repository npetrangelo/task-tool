<script setup lang="ts">
import { ref } from "vue";
const tasks = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <h1 class="rtitle">Recently Completed</h1>
    <div class="recent tasks standout">
        <li v-for="task in tasks">
            {{ task.message }}
        </li>
    </div>
</template>

<style>
.recent {
    display: flex;
    flex-direction: column;
}
</style>