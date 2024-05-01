<script setup lang="ts">
import Browser from "./Browser.vue";
import Editor from "./Editor.vue";

let props = defineProps({
	project: {
		type: String,
		default: ''
	}
})
</script>

<template>
	<div class="main_window_container">
		<Browser ref="browser"/>
		<Editor ref="editor"/>
	</div>
</template>

<style scoped>
.main_window_container {
	height: calc(100dvh - 3em);
	width: 100dvw;
	display: flex;
}
</style>