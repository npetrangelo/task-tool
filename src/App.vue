<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Projects from './components/Projects.vue';
import Current from './components/Current.vue';
import Recent from './components/Recent.vue';
import Starred from './components/Starred.vue';
</script>

<template>
    <div class="container">
        <div class="grid">
            <Projects/>
            <Current/>
            <Recent/>
            <Starred/>
        </div>
    </div>
</template>

<style>
div {
    border-radius: 5px;
    border: 1px solid transparent;
    border-color: #249b73;
    padding: 0em 0.5em;
    font-size: 0.95em;
    font-weight: 500;
    font-family: Arial, Helvetica, sans-serif;
    color: #0f0f0f;
    transition: border-color 0.25s;
}
</style>

<style scoped>
.row {
    justify-content: space-between;
    height: 100%;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>