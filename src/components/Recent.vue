<script setup lang="ts">
import { ref } from "vue";
const tasks = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="col">
        <h1>Recently Completed</h1>
        <div class="col tasks">
            <li v-for="task in tasks">
                {{ task.message }}
            </li>
        </div>
    </div>
</template>