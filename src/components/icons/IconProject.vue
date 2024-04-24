<script setup lang="ts">

</script>

<template>
	<div class="icon_container">
		<p>
			<slot></slot>
		</p>
	</div>
</template>

<style scoped>
.icon_container {
	aspect-ratio: 1/1;
	width: 2em;
	height: 2em;
	display: flex;
	align-items: center;
	justify-content: center;
	border: 2px solid var(--primary);
	background-color: rgba(var(--primary-rgb), 0.25);
	border-radius: 0.5em;

	p {
		margin: 0;
		padding: 0;
		height: fit-content;
		width: fit-content;
		max-width: 75%;
		overflow: hidden;
	}
}
</style>