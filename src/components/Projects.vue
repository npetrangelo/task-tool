<script setup lang="ts">
import { ref } from "vue";
const items = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <h1 class="ptitle">Projects</h1>
    <div class="projects">
        <div v-for="item in items" class="standout project">
            {{ item.message }}
        </div>
    </div>
</template>

<style>
.projects {
    margin: 10px;
}

.project {
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}
</style>