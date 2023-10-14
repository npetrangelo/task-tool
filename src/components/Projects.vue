<script setup lang="ts">
import { ref } from "vue";
const items = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="col">
        <h1>Projects</h1>
        <div v-for="item in items" class="project">
            {{ item.message }}
        </div>
    </div>
</template>

<style>
.project {
    background-color: aquamarine;
}
</style>