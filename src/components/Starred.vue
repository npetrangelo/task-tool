<script setup lang="ts">
import { ref } from "vue";
const tasks = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="starred">
        <h1>Starred Completes</h1>
        <div class="col tasks standout">
            <li v-for="task in tasks">
                {{ task.message }}
            </li>
        </div>
    </div>
</template>