<script setup lang="ts">
import Container from "./components/Container.vue";
import TopBar from "./components/TopBar.vue";
import MainWindow from "./components/MainWindow.vue";

import { invoke } from "@tauri-apps/api/tauri";

setTimeout(() => {
	invoke('close_splashscreen')
}, 1000)

function set_content(content) {

}
</script>

<template>
    <Container class="container">
	    <TopBar ref="top_bar" @file_content="set_content" @file_path="console.log"/>
	    <MainWindow ref="main"/>
    </Container>
</template>

<style scoped>

</style>
