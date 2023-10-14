<script setup lang="ts">
import { ref } from "vue";
const items = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="col">
        <h1>Projects</h1>
        <div class="col">
            <div v-for="item in items" class="standout">
                {{ item.message }}
            </div>
        </div>
    </div>
</template>

<style>
div {
    border-radius: 8px;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}
</style>