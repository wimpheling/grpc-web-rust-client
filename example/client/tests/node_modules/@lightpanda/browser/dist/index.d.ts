export { LightpandaFetchOptions } from './src/fetch.js';
export { LightpandaServeOptions } from './src/serve.js';
export declare const lightpanda: {
    fetch: (url: string, options?: import("./src/fetch.js").LightpandaFetchOptions) => Promise<string | Buffer<ArrayBufferLike>>;
    serve: (options?: import("./src/serve.js").LightpandaServeOptions) => Promise<import("child_process").ChildProcessWithoutNullStreams>;
};
