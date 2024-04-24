<script setup lang="ts">
import Container from "./components/Container.vue";
import TopBar from "./components/TopBar.vue";
import MainWindow from "./components/MainWindow.vue";

import { invoke } from "@tauri-apps/api/tauri";

setTimeout(() => {
	invoke('close_splashscreen')
}, 1000)
</script>

<template>
    <Container class="container">
	    <TopBar />
	    <MainWindow />
    </Container>
</template>

<style scoped>

</style>
