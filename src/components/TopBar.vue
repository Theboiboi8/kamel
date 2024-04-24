<script setup lang="ts">
import IconKamel from "./icons/IconKamel.vue";
import Title from "./Title.vue";
import IconProject from "./icons/IconProject.vue";
import {ref} from "vue";

let props = defineProps({
	project: {
		type: String,
		default: ''
	}
})

let title = ref(props.project?.valueOf().trim())
if (title.value.length >= 40) {
	title = ref(title.value.slice(0, 20).padEnd(23, '.'))
}
</script>

<template>
	<div class="topbar_container">
		<a href="/">
			<IconKamel />
			<Title>Kamel</Title>
		</a>
		<div class="project_view" v-show='title != ""'>
			<IconProject>
				{{ title.toUpperCase().slice(0, 2) }}
			</IconProject>
			<Title>{{ title }}</Title>
		</div>
	</div>
</template>

<style scoped>
.topbar_container {
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
	}
}
</style>