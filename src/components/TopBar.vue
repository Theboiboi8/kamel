<script setup lang="ts">
import IconKamel from "./icons/IconKamel.vue";
import Title from "./Title.vue";
import IconProject from "./icons/IconProject.vue";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

let props = defineProps({
	project: {
		type: String,
		default: 'TEST'
	}
})

let emit = defineEmits({
	file_content: String,
	file_path: String,
})

let title = ref(props.project?.valueOf().trim())
if (title.value?.length >= 18) {
	title = ref(title.value?.slice(0, 17).padEnd(20, '.'))
}

function pick_file() {
	invoke('pick_file').then(emit_signals)
}

function emit_signals(e: any) {
	emit('file_content', e[0]);
	emit('file_path', e[1]);
}
</script>

<template>
	<div class="top_bar_container">
		<a href="/">
			<IconKamel />
			<Title>Kamel</Title>
		</a>
		<div class="project_view" v-show='title != ""' @click="pick_file()">
			<IconProject>
				{{ title.toUpperCase().slice(0, 2) }}
			</IconProject>
			<Title>{{ title }}</Title>
		</div>
	</div>
</template>

<style scoped>
.top_bar_container {
	background-color: #121212;
	height: 3em;
	width: 100dvw;
	border-bottom: 3px solid #232323;
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 0 0.5dvw;
	gap: 1em;

	a {
		display: flex;
		align-items: center;
		gap: 0.5em;
		height: fit-content;
		width: fit-content;
	}

	.project_view {
		display: flex;
		align-items: center;
		gap: 0.5em;
		height: fit-content;
		width: fit-content;
		flex-direction: row-reverse;
		cursor: pointer;
	}
}
</style>