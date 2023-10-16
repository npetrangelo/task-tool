<script setup lang="ts">
import { ref } from "vue";
const tasks = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="recent">
        <h1>Recently&nbspCompleted</h1>
        <div class="tasks standout">
            <li v-for="task in tasks">
                {{ task.message }}
            </li>
        </div>
    </div>
</template>

<style>
.recent {
    display: flex;
    flex-direction: column;
}
</style>