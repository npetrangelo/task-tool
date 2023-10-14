<script setup lang="ts">
import { ref } from "vue";
const items = ref([{ message: 'Foo' }, { message: 'Bar' }]);
</script>

<template>
    <div class="col">
        <h1>Projects</h1>
        <p v-for="item in items">
            {{ item.message }}
        </p>
    </div>
</template>