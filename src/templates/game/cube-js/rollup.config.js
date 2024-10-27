import { nodeResolve } from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'
import rune from '@rune-runtime/rollup-plugin-rune'

export default {
	input: 'game.js',
	output: {
		file: 'dist/game.js',
		format: 'esm'
	},
	plugins: [
		commonjs(), 
		nodeResolve(),
		rune()
	]
}
