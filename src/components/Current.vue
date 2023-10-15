<script setup lang="ts">
import { ref } from "vue";
const tasks = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <h1 class="ctitle">Current&nbspTasks</h1>
    <div class="current tasks standout">
        <li v-for="task in tasks">
            {{ task.message }}
        </li>
    </div>
</template>

<style>
.current {
    display: flex;
    flex-direction: column;
}
.tasks {
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    text-align: left;
    padding-left: 2vh;
    margin: 10px;
}
</style>