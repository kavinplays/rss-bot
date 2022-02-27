import preprocess from "svelte-preprocess";
import adapter_static from '@sveltejs/adapter-static'
/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        adapter: adapter_static()
	},

    preprocess: [preprocess({
        "postcss": true
    })],

};

export default config;
