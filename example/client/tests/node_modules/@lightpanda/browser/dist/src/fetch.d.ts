/**
 * @typedef LightpandaFetchOptions
 * @type {object}
 * @property {boolean} dump - Export fetched output as string
 * @property {boolean} dumpHtml - Export fetched output as HTML
 * @property {boolean} dumpMarkdown - Export fetched output as Markdown
 * @property {boolean} disableHostVerification - Disables host verification on all HTTP requests
 * @property {boolean} obeyRobots - Fetches and obeys the robots.txt (if available) of the web pages we make requests towards.
 * @property {string} httpProxy - The HTTP proxy to use for all HTTP requests
 */
export type LightpandaFetchOptions = {
    disableHostVerification?: boolean;
    obeyRobots?: boolean;
    httpProxy?: string;
    dump?: boolean;
    dumpOptions?: {
        type?: 'html' | 'markdown';
    };
};
/**
 * Fetch data from a URL
 * @param {string} url - URL to fetch data from
 * @param {LightpandaFetchOptions} options - Additional options to pass to Lightpanda
 * @returns {Promise<Buffer | string>}
 */
export declare const fetch: (url: string, options?: LightpandaFetchOptions) => Promise<string | Buffer<ArrayBufferLike>>;
