<script setup lang="ts">
import { ref } from "vue";
const tasks = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="starred">
        <h1>Starred Completes</h1>
        <div class="tasks standout">
            <li v-for="task in tasks">
                {{ task.message }}
            </li>
        </div>
    </div>
</template>

<style>
.starred {
    display: flex;
    flex-direction: column;
}
</style>

<style scoped>
.tasks {
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    text-align: left;
    padding-left: 2vh;
    margin: 0px;
}
</style>