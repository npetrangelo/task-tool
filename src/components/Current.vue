<script setup lang="ts">
import { ref } from "vue";
const tasks = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="col current">
        <h1>Current Tasks</h1>
        <div class="tasks standout">
            <li v-for="task in tasks">
                {{ task.message }}
            </li>
        </div>
    </div>
</template>

<style>
.current {
    width: 100%;
}
.tasks {
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    text-align: left;
    padding-left: 2vh;
    height: 80%;
}
</style>