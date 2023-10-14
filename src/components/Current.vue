<script setup lang="ts">
import { ref } from "vue";
const tasks = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="col">
        <h1>Current Tasks</h1>
        <div class="tasks">
            <li v-for="task in tasks">
                {{ task.message }}
            </li>
        </div>
    </div>
</template>

<style>
.tasks {
    border-radius: 10px;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    text-align: left;
    padding-left: 2vh;
    height: 100%;
}
</style>