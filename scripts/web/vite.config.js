/** @type {import('vite').UserConfig} */
export default {
    build: {
        outDir: "../../assets",
        rollupOptions: {
            output: {
                entryFileNames: `assets/[name].js`,
                chunkFileNames: `assets/[name].js`,
                assetFileNames: `assets/[name].[ext]`
            }
        }
    },
    base: "./", // So that the generated index.html refers js bundle with relative path
}