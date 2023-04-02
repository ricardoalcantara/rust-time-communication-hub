<script lang="ts">
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import { makeid } from '../lib/utils';
	const sse = writable<{ connected: boolean; messages: string[] }>({
		connected: false,
		messages: []
	});

	let server = `http://localhost:4500/sse?user_id=${makeid(5)}`;
	let message: string;
	let eventSource: EventSource;
	onMount(() => {
		eventSource = new EventSource(server);
		eventSource.onmessage = (e) => {
			console.log(e.data);
			if (e.data !== 'ping') {
				sse.update((s) => {
					s.messages.push(e.data);
					return s;
				});
			}
		};

		return () => eventSource.close();
	});

	async function sendMessage() {
		fetch(`http://localhost:4500/notify?user_id=dddd`)
			.then((response) => response.text())
			.then((data) => console.log(data));
	}
</script>

<div class="flex flex-col p-2 border">
	<h2>Chat</h2>
	<input bind:value={server} class="border border-black rounded-sm" type="text" />
	<div>Total: {$sse.messages.length} mensagens</div>
	<ul class="h-full">
		{#each $sse.messages as message, i (i)}
			<li>{i} - {message}</li>
		{/each}
	</ul>

	<input bind:value={message} class="border border-black rounded-sm" type="text" />
	<button on:click={sendMessage}>Send</button>
</div>
