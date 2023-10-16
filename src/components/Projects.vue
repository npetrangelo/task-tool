<script setup lang="ts">
import { ref } from "vue";
const items = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="projects">
        <h1>Projects</h1>
        <div class="col">
            <div v-for="item in items" class="standout project">
                {{ item.message }}
            </div>
        </div>
    </div>
</template>

<style>
.project {
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}
</style>